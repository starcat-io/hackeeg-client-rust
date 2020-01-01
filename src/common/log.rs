pub fn setup_logger(
    level: log::LevelFilter,
    maybe_output_file: Option<&std::path::Path>,
) -> Result<(), fern::InitError> {
    let mut logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[Thread: {:?}][{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                std::thread::current().id(),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout());

    if let Some(output_file) = maybe_output_file {
        logger = logger.chain(fern::log_file(output_file)?);
    }

    logger.apply()?;

    Ok(())
}
