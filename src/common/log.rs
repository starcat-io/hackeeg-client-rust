// Copyright © 2020 Starcat LLC
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
