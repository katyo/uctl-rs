/*!

## PT1 filter

This module implements first order proportional transmission behavior filter.

See also [PT1](https://de.wikipedia.org/wiki/PT1-Glied) article.

 */

use crate::{FromOther, Transducer};

/// PT1 parameters
#[derive(Debug, Clone, Copy)]
pub struct Param<K> {
    /// The time factor of filter
    kx: K,
    ///
}
