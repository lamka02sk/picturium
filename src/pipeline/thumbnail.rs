use std::path::Path;

use libvips::{ops, VipsImage};
use libvips::ops::ThumbnailOptions;

use crate::parameters::UrlParameters;
use crate::pipeline::{PipelineError, PipelineResult};
use crate::services::formats::is_thumbnail_format;
use crate::services::vips::get_error_message;

pub(crate) async fn run(working_file: &Path, url_parameters: &UrlParameters<'_>) -> PipelineResult<VipsImage> {

    let image = match VipsImage::new_from_file(&working_file.to_string_lossy()) {
        Ok(image) => image,
        Err(error) => return Err(PipelineError(format!("Failed to open image: {}", error)))
    };

    if !is_thumbnail_format(url_parameters.path) {
        return Ok(image);
    }

    match ops::thumbnail_with_opts(&working_file.to_string_lossy(), url_parameters.width.unwrap_or(1000) as i32, &ThumbnailOptions {
        height: 5000,
        import_profile: "sRGB".to_string(),
        export_profile: "sRGB".to_string(),
        ..Default::default()
    }) {
        Ok(image) => Ok(image),
        _ => Err(PipelineError(format!("Failed to generate thumbnail for PDF file {working_file:?}: {}", get_error_message())))
    }

}