# Environment Variables
Retina uses a few different environment variables to enable and disable some features at runtime.

## RETINA_TRACE
This environment variable specifies whether or not trace files must be generated. These trace
files are in the [format][format] of [`chrome://tracing`][chrome-tracing].\
**Default Value:** `0`

## RETINA_URL
This environment variable contains the hyperlink (URL) that the browser should visit.\
**Default Value:** `about:not-found`

[chrome-tracing]: https://ui.perfetto.dev/
[format]: https://www.chromium.org/developers/how-tos/trace-event-profiling-tool/
