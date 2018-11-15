//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use std::iter::FromIterator;

/// Extension trait for `Iterator<Item = Result<T, E>>` to collect values if there is no error.
pub trait TryCollectExt<T, E> {
    fn try_collect<B>(&mut self) -> Result<B, E>
    where
        B: FromIterator<T>;
}

impl<T, E, I> TryCollectExt<T, E> for I
where
    I: Iterator<Item=Result<T,E>>
{
    fn try_collect<B>(&mut self) -> Result<B, E>
    where
        B: FromIterator<T>
    {
        let mut error: Option<E> = None;
        let collected = self
        .map(|i| match i {
            Ok(v)  => Some(v),
            Err(e) => {
                error = Some(e);
                None
            }
        })
        .take_while(|i| i.is_some())
        .map(|i| i.unwrap())
        .collect();

        match error {
            None => Ok(collected),
            Some(e) => Err(e)
        }
    }
}

#[test]
fn test_try_collect() {
    let r1: Result<Vec<usize>, &'static str> =
        vec![ Ok(1), Ok(2), Ok(3), Err("error"), Ok(4) ]
        .into_iter()
        .try_collect();
    assert!(r1.is_err());

    let r2: Result<Vec<usize>, &'static str> =
        vec![ Ok(1), Ok(2), Ok(3), Ok(4) ]
        .into_iter()
        .try_collect();
    assert!(r2.is_ok());
    assert!(r2.unwrap().len() == 4);
}

