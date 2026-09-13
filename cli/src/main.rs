use anyhow::{bail, Context, Result};
use chrono::Utc;
use clap::{Parser, Subcommand};
use ptc::deposit::{deposit, DepositSummary};
use ptc::encrypt::{encrypt, EncryptSummary};
use ptc::import::{import, ImportSummary};
use ptc::inspect::{inspect, InspectReport};
use ptc::model::{
    self, Access, Capsule, CapsuleIdentity, Creator, Integrity, LifecycleStage, ManifestAlgorithm,
    PrivacyZone, Purpose, ReleasePolicy, ReleasePolicyType, Status, SPEC_VERSION,
};
use ptc::redact::{redact, RedactSummary};
use ptc::scan::{scan, ScanReport};
use ptc::seal::{seal, SealSummary};
use ptc::unlock::{unlock, UnlockSummary};
use ptc::verify::verify;
use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "ptc", version, about = "Personal Time Capsule reference CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Scaffold a new, empty capsule at the given path.
    Init(InitArgs),
    /// Copy a file or directory tree into a capsule's data/<collection>/,
    /// registering the collection and logging a provenance event.
    Import(ImportArgs),
    /// Print a read-only summary of a capsule's current state: collections,
    /// the human-context layer, provenance, and optional tag files.
    Inspect(InspectArgs),
    /// Scan a capsule's payload for candidate sensitive-data findings
    /// (SSNs, credit card numbers, credentials). Report only -- writes
    /// nothing.
    Scan(ScanArgs),
    /// Redact candidate sensitive-data findings in one file. Dry run by
    /// default -- pass --apply to actually rewrite the file.
    Redact(RedactArgs),
    /// Seal a capsule: compute manifest-sha256.txt / tagmanifest-sha256.txt,
    /// write bag-info.txt, and mark status.lifecycle_stage as "sealing".
    Seal(SealArgs),
    /// Encrypt a freshly sealed capsule's non-immediate-release payload,
    /// one key per collection. Dry run by default -- pass --apply to
    /// actually encrypt. Key-splitting (SLIP-0039) is not implemented yet;
    /// --apply prints each unit's raw key for the creator to record.
    Encrypt(EncryptArgs),
    /// Recompute every payload/tag file's checksum and compare against
    /// manifest-sha256.txt / tagmanifest-sha256.txt. Read-only.
    Verify(VerifyArgs),
    /// Decrypt one unit's .enc files back to plaintext, given the key
    /// ptc encrypt --apply issued for it. Dry run by default -- pass
    /// --apply to actually rewrite the unit.
    Unlock(UnlockArgs),
    /// Verify a sealed capsule, optionally archive it into one .tar.gz,
    /// and record a deposit event naming the custodian. Dry run by
    /// default -- pass --apply to actually archive/log.
    Deposit(DepositArgs),
}

#[derive(clap::Args)]
struct InitArgs {
    /// Directory to create the capsule in. Must not already exist (or must be empty).
    path: PathBuf,

    /// Creator's name, written into capsule.yaml's capsule.creator.name.
    /// Optional per PRINCIPLES.md #2 -- the creator controls what
    /// identifying information appears here at all.
    #[arg(long = "creator-name")]
    creator_name: Option<String>,

    /// Creator's contact info (email, etc.), written into capsule.creator.contact.
    #[arg(long = "creator-contact")]
    creator_contact: Option<String>,

    /// Free-text description of this capsule's purpose.
    #[arg(long)]
    purpose: Option<String>,
}

#[derive(clap::Args)]
struct ImportArgs {
    /// Path to the capsule directory to import into.
    path: PathBuf,

    /// File or directory to copy in. A directory is nested under its own
    /// name inside the collection (data/<collection>/<dir-name>/...), so
    /// importing several source directories into one collection (e.g. a
    /// "1945" and a "1946" photo folder both going into "photos") can't
    /// collide.
    #[arg(long)]
    source: PathBuf,

    /// Name of the collection to import into (e.g. "photos", "email").
    /// Registered in capsule.yaml automatically if not already declared
    /// there.
    #[arg(long)]
    collection: String,

    /// Free-text provenance description for the import event (e.g. "Gmail
    /// Takeout export, jane@example.org"). Defaults to a description
    /// naming the --source path.
    #[arg(long = "source-description")]
    source_description: Option<String>,

    /// Who/what performed this import, recorded in the provenance event.
    /// Defaults to "ptc CLI v<version>".
    #[arg(long)]
    agent: Option<String>,
}

#[derive(clap::Args)]
struct InspectArgs {
    /// Path to the capsule directory to inspect.
    path: PathBuf,
}

#[derive(clap::Args)]
struct ScanArgs {
    /// Path to the capsule directory to scan.
    path: PathBuf,

    /// Limit the scan to one named collection instead of all of data/.
    #[arg(long)]
    collection: Option<String>,
}

#[derive(clap::Args)]
struct RedactArgs {
    /// Path to the capsule directory containing the file to redact.
    path: PathBuf,

    /// The file to redact, relative to the capsule root (e.g.
    /// "data/email/2020-taxes.eml").
    #[arg(long)]
    target: String,

    /// Actually rewrite the file. Without this, ptc redact only reports
    /// what it would do (identical to `ptc scan` for this one file).
    #[arg(long)]
    apply: bool,
}

#[derive(clap::Args)]
struct SealArgs {
    /// Path to the capsule directory to seal. Must already contain a
    /// capsule.yaml (see `ptc init`) with a real release policy set.
    path: PathBuf,
}

#[derive(clap::Args)]
struct EncryptArgs {
    /// Path to the capsule directory to encrypt. Must already be sealed.
    path: PathBuf,

    /// Actually encrypt. Without this, ptc encrypt only reports which
    /// units would be encrypted and how many files each has, without
    /// generating any keys or touching any files.
    #[arg(long)]
    apply: bool,
}

#[derive(clap::Args)]
struct VerifyArgs {
    /// Path to the capsule directory to verify. Must already be sealed.
    path: PathBuf,
}

#[derive(clap::Args)]
struct UnlockArgs {
    /// Path to the capsule directory to unlock.
    path: PathBuf,

    /// Name of the unit to unlock (a collection name, or "person"/
    /// "provenance"), matching what `ptc encrypt` reported for it.
    #[arg(long)]
    collection: String,

    /// The unit's key, hex-encoded, exactly as `ptc encrypt --apply`
    /// printed it. If omitted, read one line from stdin instead --
    /// safer than a flag when the key shouldn't end up in shell history.
    #[arg(long)]
    key: Option<String>,

    /// Actually decrypt and remove the ciphertext. Without this, ptc
    /// unlock only confirms the key decrypts every file in the unit,
    /// without writing or removing anything.
    #[arg(long)]
    apply: bool,
}

#[derive(clap::Args)]
struct DepositArgs {
    /// Path to the capsule directory to deposit. Must already be sealed
    /// and pass verification.
    path: PathBuf,

    /// Name of the custodian institution accepting this deposit, recorded
    /// as the deposit event's agent.
    #[arg(long)]
    custodian: String,

    /// Also serialize the capsule into one .tar.gz at this path, purely
    /// as a transport convenience (BagIt's own "serialization" concept)
    /// -- must be outside the capsule directory itself.
    #[arg(long)]
    archive: Option<PathBuf>,

    /// Actually write the archive (if requested) and log the deposit
    /// event. Without this, ptc deposit only verifies and reports what
    /// it would do.
    #[arg(long)]
    apply: bool,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init(args) => cmd_init(&args),
        Command::Import(args) => cmd_import(&args),
        Command::Inspect(args) => cmd_inspect(&args),
        Command::Scan(args) => cmd_scan(&args),
        Command::Redact(args) => cmd_redact(&args),
        Command::Seal(args) => cmd_seal(&args),
        Command::Encrypt(args) => cmd_encrypt(&args),
        Command::Verify(args) => cmd_verify(&args),
        Command::Unlock(args) => cmd_unlock(&args),
        Command::Deposit(args) => cmd_deposit(&args),
    }
}

fn cmd_init(args: &InitArgs) -> Result<()> {
    let path = &args.path;

    if path.exists() {
        if path
            .read_dir()
            .with_context(|| format!("reading {}", path.display()))?
            .next()
            .is_some()
        {
            bail!("{} already exists and is not empty", path.display());
        }
    } else {
        fs::create_dir_all(path).with_context(|| format!("creating {}", path.display()))?;
    }

    let data_person = path.join("data").join("person");
    fs::create_dir_all(&data_person)
        .with_context(|| format!("creating {}", data_person.display()))?;
    write_person_templates(&data_person, args.creator_name.as_deref())?;

    let capsule = build_new_capsule(args);
    let header = "# Generated by `ptc init` (Personal Time Capsule reference CLI).\n\
                  # See FORMAT.md for the full field reference:\n\
                  # https://github.com/neodem/personal-time-capsule/blob/main/FORMAT.md\n\n";
    model::write_capsule_yaml(&path.join("capsule.yaml"), &capsule, header)?;

    fs::write(
        path.join("bagit.txt"),
        "BagIt-Version: 1.0\nTag-File-Character-Encoding: UTF-8\n",
    )
    .context("writing bagit.txt")?;

    println!("Initialized a new capsule at {}", path.display());
    println!();
    println!("This is a capsule SKELETON, not yet a complete BagIt bag --");
    println!("no manifest-sha256.txt / tagmanifest-sha256.txt / bag-info.txt");
    println!("are written yet. Those are written by `ptc seal` (LIFECYCLE.md");
    println!("stage 5), once you're ready.");
    println!();
    println!("Next: fill in data/person/ (biography.md, timeline.md, people.yaml,");
    println!("places.yaml -- starter templates already written there), add");
    println!("content under data/, then edit capsule.yaml -- in particular,");
    println!("access.default_release_policy is a placeholder ('custom' / TBD)");
    println!("since `ptc init` can't choose a real release policy on your");
    println!("behalf. `ptc seal` will refuse to run until you've set a real one.");

    Ok(())
}

/// Write starter templates for FORMAT.md's `data/person/` human-context
/// layer (biography, timeline, people, places) -- previously `ptc init`
/// created only the empty directory, silently omitting the four files
/// FORMAT.md's own directory listing documents as belonging there. These
/// are genuinely empty (guidance text, not fictional example content, per
/// PRINCIPLES.md #2 -- the creator's own life doesn't get invented for
/// them) but structurally real, so a creator opens files that already
/// exist and already show the expected shape, rather than an empty folder
/// with no clue what belongs in it.
fn write_person_templates(data_person: &Path, creator_name: Option<&str>) -> Result<()> {
    let biography_title = match creator_name {
        Some(name) => format!("# Biography — {name}"),
        None => "# Biography".to_string(),
    };
    fs::write(
        data_person.join("biography.md"),
        format!(
            "{biography_title}\n\n\
             Write a short account of who you are: where you grew up, what you did, \
             who and what mattered to you -- whatever you'd want a stranger reading \
             this decades from now to understand about your life. There's no \
             required length or structure; a few paragraphs or many, your call. See \
             CREATOR_GUIDE.md.\n"
        ),
    )
    .context("writing data/person/biography.md")?;

    fs::write(
        data_person.join("timeline.md"),
        "# Timeline\n\n\
         A rough chronology of your life -- the dates and events that mattered to \
         you. One line per entry is plenty; add more later as it comes to you, \
         e.g.:\n\n\
         - YYYY -- what happened\n",
    )
    .context("writing data/person/timeline.md")?;

    fs::write(
        data_person.join("people.yaml"),
        "# Who's who -- the people who show up in this capsule and how they relate\n\
         # to you. See FORMAT.md. One entry per person, e.g.:\n\
         #\n\
         # - id: jane-doe\n\
         #   relationship: spouse\n\
         #   note: A short note about who they are to you.\n\
         \n\
         []\n",
    )
    .context("writing data/person/people.yaml")?;

    fs::write(
        data_person.join("places.yaml"),
        "# Places that mattered -- where you lived, worked, or that show up\n\
         # elsewhere in this capsule. One entry per place, e.g.:\n\
         #\n\
         # - id: hometown\n\
         #   name: Springfield, Anystate\n\
         #   note: A short note about why this place matters.\n\
         \n\
         []\n",
    )
    .context("writing data/person/places.yaml")?;

    Ok(())
}

fn build_new_capsule(args: &InitArgs) -> Capsule {
    let id = format!("urn:uuid:{}", Uuid::new_v4());
    let created = Utc::now().format("%Y-%m-%d").to_string();

    let creator = if args.creator_name.is_some() || args.creator_contact.is_some() {
        Some(Creator {
            name: args.creator_name.clone(),
            contact: args.creator_contact.clone(),
            extra: Default::default(),
        })
    } else {
        None
    };

    let purpose = args.purpose.as_ref().map(|description| Purpose {
        description: Some(description.clone()),
        extra: Default::default(),
    });

    Capsule {
        capsule: CapsuleIdentity {
            spec_version: SPEC_VERSION.to_string(),
            id,
            created,
            creator,
        },
        purpose,
        series: None,
        collections: None,
        access: Access {
            // Defaulted to the more conservative zone; the creator edits this
            // once they've actually decided -- see FORMAT.md's `access` section.
            default_privacy_zone: PrivacyZone::Restricted,
            default_release_policy: ReleasePolicy {
                // `ptc init` cannot choose a release policy on the creator's
                // behalf; `ptc seal` enforces that this placeholder gets
                // replaced before a capsule can actually be sealed.
                policy_type: ReleasePolicyType::Custom,
                date: None,
                years: None,
                description: Some("TBD -- set by the creator".to_string()),
            },
        },
        custody: None,
        encryption: None,
        integrity: Integrity {
            manifest_algorithm: ManifestAlgorithm::Sha256,
        },
        status: Status {
            lifecycle_stage: LifecycleStage::Origination,
        },
    }
}

fn cmd_import(args: &ImportArgs) -> Result<()> {
    let ImportSummary {
        files_copied,
        bytes_copied,
        collection,
        collection_path,
        collection_registered,
    } = import(
        &args.path,
        &args.source,
        &args.collection,
        args.source_description.as_deref(),
        args.agent.as_deref(),
    )?;

    println!(
        "Imported {} file(s) ({} bytes) from {} into {}/",
        files_copied,
        bytes_copied,
        args.source.display(),
        collection_path
    );
    if collection_registered {
        println!(
            "Registered new collection \"{}\" in capsule.yaml.",
            collection
        );
    }
    println!("Logged an import event in data/provenance/events.yaml.");

    Ok(())
}

fn cmd_inspect(args: &InspectArgs) -> Result<()> {
    let InspectReport {
        capsule,
        collections,
        undeclared_directories,
        person_layer,
        provenance,
        privacy_overrides_count,
        legal_files,
        preservation_log_entries,
        manifest_present,
        tagmanifest_present,
        total_payload_files,
        total_payload_bytes,
    } = inspect(&args.path)?;

    println!("Capsule at {}", args.path.display());
    println!("  id:      {}", capsule.capsule.id);
    println!("  created: {}", capsule.capsule.created);
    if let Some(creator) = &capsule.capsule.creator {
        let name = creator.name.as_deref().unwrap_or("(name not set)");
        match &creator.contact {
            Some(contact) => println!("  creator: {name} <{contact}>"),
            None => println!("  creator: {name}"),
        }
    }
    if let Some(purpose) = capsule
        .purpose
        .as_ref()
        .and_then(|p| p.description.as_ref())
    {
        println!("  purpose: {purpose}");
    }
    println!(
        "  stage:   {}",
        lifecycle_stage_str(capsule.status.lifecycle_stage)
    );

    println!();
    println!("Human-context layer (data/person/):");
    println!(
        "  biography.md: {}",
        if person_layer.biography_present {
            "present"
        } else {
            "missing"
        }
    );
    println!(
        "  timeline.md:  {}",
        if person_layer.timeline_present {
            "present"
        } else {
            "missing"
        }
    );
    println!(
        "  people.yaml:  {}",
        describe_count(person_layer.people_count, "person", "people")
    );
    println!(
        "  places.yaml:  {}",
        describe_count(person_layer.places_count, "place", "places")
    );

    println!();
    println!("Provenance (data/provenance/events.yaml):");
    match provenance {
        Some(p) => {
            print!("  {} event(s) logged", p.event_count);
            if let Some(recent) = p.most_recent {
                println!(
                    "; most recent: {} {} (agent: {})",
                    recent.date, recent.event_type, recent.agent
                );
            } else {
                println!();
            }
        }
        None => println!("  not present"),
    }

    if !collections.is_empty() {
        println!();
        println!("Collections:");
        for c in &collections {
            let existence = if c.exists { "" } else { " [directory missing]" };
            println!(
                "  {:<15} {:<20} {} file(s), {} bytes{}",
                c.name, c.path, c.file_count, c.total_bytes, existence
            );
            println!(
                "    privacy: {}   release: {}",
                c.privacy_zone, c.release_policy
            );
        }
    }

    if !undeclared_directories.is_empty() {
        println!();
        println!("Undeclared directories under data/ (not registered as a collection):");
        for d in &undeclared_directories {
            println!(
                "  {:<15} {} file(s), {} bytes",
                d.name, d.file_count, d.total_bytes
            );
        }
    }

    println!();
    println!("Other tag files:");
    println!(
        "  privacy-overrides.yaml: {}",
        describe_count(privacy_overrides_count, "override", "overrides")
    );
    match legal_files {
        Some(files) if !files.is_empty() => {
            println!(
                "  legal/:                  {} file(s) ({})",
                files.len(),
                files.join(", ")
            )
        }
        Some(_) => println!("  legal/:                  present, empty"),
        None => println!("  legal/:                  not present"),
    }
    println!(
        "  preservation-log.yaml:   {}",
        describe_count(preservation_log_entries, "entry", "entries")
    );

    println!();
    println!("Sealing:");
    println!(
        "  manifest-sha256.txt:    {}",
        if manifest_present {
            "present"
        } else {
            "not present"
        }
    );
    println!(
        "  tagmanifest-sha256.txt: {}",
        if tagmanifest_present {
            "present"
        } else {
            "not present"
        }
    );

    println!();
    println!(
        "Total payload under data/: {} file(s), {} bytes",
        total_payload_files, total_payload_bytes
    );

    Ok(())
}

/// `capsule.yaml`'s own snake_case vocabulary (matching `FORMAT.md`/
/// `METADATA.md`), not Rust's `{:?}` variant spelling -- a creator
/// cross-referencing this output against the file itself shouldn't see
/// "Import" print here and "import" there for the same thing.
fn lifecycle_stage_str(stage: LifecycleStage) -> &'static str {
    match stage {
        LifecycleStage::Origination => "origination",
        LifecycleStage::Import => "import",
        LifecycleStage::Annotation => "annotation",
        LifecycleStage::Screening => "screening",
        LifecycleStage::Sealing => "sealing",
        LifecycleStage::Encryption => "encryption",
        LifecycleStage::Deposit => "deposit",
        LifecycleStage::DormantPreservation => "dormant_preservation",
        LifecycleStage::ReleaseTrigger => "release_trigger",
        LifecycleStage::Unlock => "unlock",
        LifecycleStage::Access => "access",
    }
}

fn describe_count(count: Option<usize>, singular: &str, plural: &str) -> String {
    match count {
        Some(1) => format!("1 {singular}"),
        Some(n) => format!("{n} {plural}"),
        None => "not present".to_string(),
    }
}

fn cmd_scan(args: &ScanArgs) -> Result<()> {
    let ScanReport {
        scan_root,
        files_scanned,
        files_skipped_binary,
        findings,
    } = scan(&args.path, args.collection.as_deref())?;

    println!(
        "Scanned {} file(s) under {}/ ({} skipped as binary).",
        files_scanned, scan_root, files_skipped_binary
    );
    println!();

    if findings.is_empty() {
        println!("No findings.");
    } else {
        println!("Findings ({}):", findings.len());
        for f in &findings {
            let snippet = match &f.snippet {
                Some(s) => format!(": {s}"),
                None => String::new(),
            };
            println!(
                "  {}:{}  [{}]  {}{}",
                f.path, f.line, f.category, f.description, snippet
            );
        }

        println!();
        println!("By category:");
        for category in ["financial", "credentials", "government-id"] {
            let count = findings.iter().filter(|f| f.category == category).count();
            if count > 0 {
                println!("  {category}: {count}");
            }
        }
    }

    println!();
    println!("Nothing here is written automatically. `ptc scan` only surfaces");
    println!("candidates -- per PRINCIPLES.md #2, deciding what's actually");
    println!("sensitive is yours, not the tool's. Review each finding and, if");
    println!("it's real, record it yourself in privacy-overrides.yaml (see");
    println!("PRIVACY.md) using the category shown above.");

    Ok(())
}

fn cmd_redact(args: &RedactArgs) -> Result<()> {
    let RedactSummary {
        path,
        findings,
        redactions_applied,
        ..
    } = redact(&args.path, &args.target, args.apply)?;

    if findings.is_empty() {
        println!("No candidate findings in {path} -- nothing to redact.");
        return Ok(());
    }

    println!("Findings in {path} ({}):", findings.len());
    for f in &findings {
        let snippet = match &f.snippet {
            Some(s) => format!(": {s}"),
            None => String::new(),
        };
        let tag = if f.redactable {
            ""
        } else {
            "  [MANUAL REVIEW -- not auto-redactable]"
        };
        println!(
            "  line {}  [{}]  {}{}{}",
            f.line, f.category, f.description, snippet, tag
        );
    }
    println!();

    let manual_review_count = findings.iter().filter(|f| !f.redactable).count();

    if args.apply {
        if redactions_applied > 0 {
            println!("Applied {redactions_applied} redaction(s) to {path}.");
            println!("Logged a screening event in data/provenance/events.yaml.");
        } else {
            println!("Nothing auto-redactable was found -- {path} left unchanged.");
        }
        if manual_review_count > 0 {
            println!(
                "{manual_review_count} finding(s) need manual review (see above) -- \
                 ptc redact does not modify these."
            );
        }
    } else {
        let auto_redactable_count = findings.len() - manual_review_count;
        println!(
            "Dry run -- {path} was not modified. Re-run with --apply to redact the \
             {auto_redactable_count} auto-redactable finding(s) above in place."
        );
    }

    Ok(())
}

fn cmd_seal(args: &SealArgs) -> Result<()> {
    let SealSummary {
        payload_files_hashed,
        payload_bytes,
        tag_files_hashed,
    } = seal(&args.path)?;

    println!("Sealed capsule at {}", args.path.display());
    println!();
    println!(
        "  manifest-sha256.txt:    {} payload file(s), {} bytes",
        payload_files_hashed, payload_bytes
    );
    println!(
        "  tagmanifest-sha256.txt: {} tag file(s) ({})",
        tag_files_hashed.len(),
        tag_files_hashed.join(", ")
    );
    println!();
    println!("capsule.yaml's status.lifecycle_stage is now \"sealing\". Any");
    println!("change to data/ or the sealed tag files after this point should");
    println!("be treated as suspect -- re-verify against these manifests.");

    Ok(())
}

fn cmd_encrypt(args: &EncryptArgs) -> Result<()> {
    let EncryptSummary {
        units,
        skipped_immediate,
    } = encrypt(&args.path, args.apply)?;

    if !args.apply {
        println!("Dry run -- nothing was encrypted, no keys were generated.");
        println!();
    }

    for u in &units {
        if u.already_encrypted {
            println!(
                "{:<12} ({}): already encrypted -- nothing to do.",
                u.name, u.path
            );
            continue;
        }
        if args.apply {
            println!(
                "{:<12} ({}): {} file(s) encrypted.",
                u.name, u.path, u.files_encrypted
            );
            let key_hex = u
                .key_hex
                .as_deref()
                .expect("apply always issues a key for a newly-encrypted unit");
            println!("  KEY (xchacha20-poly1305, 256-bit, hex) -- RECORD THIS NOW, IT WILL NOT BE SHOWN AGAIN:");
            println!("  {key_hex}");
            println!(
                "  SLIP-0039 key-splitting is not implemented yet (see cli/README.md) -- \
                 you are responsible for recording and distributing this key yourself."
            );
        } else {
            println!(
                "{:<12} ({}): {} file(s) would be encrypted.",
                u.name, u.path, u.files_encrypted
            );
        }
    }

    for name in &skipped_immediate {
        println!("{name:<12}: release_policy is immediate -- skipped, never encrypted.");
    }

    println!();
    if args.apply {
        let encrypted_count = units.iter().filter(|u| !u.already_encrypted).count();
        if encrypted_count > 0 {
            println!("Logged {encrypted_count} encryption event(s) in preservation-log.yaml.");
        } else {
            println!("Nothing new to encrypt.");
        }
    } else {
        println!("Re-run with --apply to actually encrypt.");
    }

    Ok(())
}

fn cmd_verify(args: &VerifyArgs) -> Result<()> {
    let report = verify(&args.path)?;

    println!("Verifying capsule at {}", args.path.display());
    println!();
    println!("Payload (manifest-sha256.txt):");
    println!(
        "  {} file(s) checked, {} OK",
        report.payload_checked, report.payload_ok
    );
    if !report.payload_encrypted.is_empty() {
        println!(
            "  {} encrypted -- not independently verifiable until unlock (see ENCRYPTION.md):",
            report.payload_encrypted.len()
        );
        for p in &report.payload_encrypted {
            println!("    {p}");
        }
    }
    if !report.payload_missing.is_empty() {
        println!("  {} MISSING:", report.payload_missing.len());
        for p in &report.payload_missing {
            println!("    {p}");
        }
    }
    if !report.payload_mismatched.is_empty() {
        println!("  {} HASH MISMATCH:", report.payload_mismatched.len());
        for p in &report.payload_mismatched {
            println!("    {p}");
        }
    }

    println!();
    println!("Tag files (tagmanifest-sha256.txt):");
    println!(
        "  {} file(s) checked, {} OK",
        report.tag_checked, report.tag_ok
    );
    if !report.tag_missing.is_empty() {
        println!("  {} MISSING:", report.tag_missing.len());
        for p in &report.tag_missing {
            println!("    {p}");
        }
    }
    if !report.tag_mismatched.is_empty() {
        println!("  {} HASH MISMATCH:", report.tag_mismatched.len());
        for p in &report.tag_mismatched {
            println!("    {p}");
        }
    }

    if !report.unexpected_files.is_empty() {
        println!();
        println!(
            "Unexpected file(s) under data/ not recorded in manifest-sha256.txt (added after \
             sealing without a reseal, or possible tampering):"
        );
        for p in &report.unexpected_files {
            println!("  {p}");
        }
    }

    println!();
    if report.passed() {
        println!("VERIFICATION PASSED.");
        Ok(())
    } else {
        println!("VERIFICATION FAILED -- see above.");
        bail!("capsule failed verification")
    }
}

fn cmd_unlock(args: &UnlockArgs) -> Result<()> {
    let key = match &args.key {
        Some(k) => k.clone(),
        None => {
            let stdin = std::io::stdin();
            let mut line = String::new();
            stdin
                .lock()
                .read_line(&mut line)
                .context("reading key from stdin")?;
            line.trim().to_string()
        }
    };

    let UnlockSummary {
        name,
        path,
        files_unlocked,
        applied,
    } = unlock(&args.path, &args.collection, &key, args.apply)?;

    if files_unlocked == 0 {
        println!("{name} ({path}): no encrypted (.enc) files found -- nothing to unlock.");
        return Ok(());
    }

    if applied {
        println!("{name} ({path}): {files_unlocked} file(s) decrypted.");
        println!("Logged an unlock event in preservation-log.yaml.");
    } else {
        println!("{name} ({path}): key verified -- {files_unlocked} file(s) would be decrypted.");
        println!("Re-run with --apply to actually decrypt and remove the ciphertext.");
    }

    Ok(())
}

fn cmd_deposit(args: &DepositArgs) -> Result<()> {
    let DepositSummary {
        verify_report,
        archive_path,
        applied,
    } = deposit(
        &args.path,
        &args.custodian,
        args.archive.as_deref(),
        args.apply,
    )?;

    println!(
        "Verification passed: {} payload file(s) OK, {} tag file(s) OK{}.",
        verify_report.payload_ok,
        verify_report.tag_ok,
        if verify_report.payload_encrypted.is_empty() {
            String::new()
        } else {
            format!(
                ", {} encrypted (not independently verifiable until unlock)",
                verify_report.payload_encrypted.len()
            )
        }
    );

    if applied {
        if let Some(path) = &archive_path {
            println!("Archived capsule to {path}.");
        }
        println!(
            "Logged a deposit event in preservation-log.yaml (custodian: {}).",
            args.custodian
        );
    } else {
        match &archive_path {
            Some(path) => println!("Dry run -- would archive to {path} and log a deposit event."),
            None => println!("Dry run -- would log a deposit event (no archive requested)."),
        }
        println!("Re-run with --apply to actually do this.");
    }

    Ok(())
}
