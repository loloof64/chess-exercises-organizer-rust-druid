mod graphic;
use graphic::launch;

use druid::PlatformError;

fn main() -> Result<(), PlatformError> {
    launch()
}
