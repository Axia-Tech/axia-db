// Copyright 2015-2020 AXIA Technologies (UK) Ltd.
// This file is part of AXIA.

// AXIA is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// AXIA is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with AXIA.  If not, see <http://www.gnu.org/licenses/>.

mod db;
mod error;
mod index;
mod table;
mod column;
mod file;
mod log;
mod display;
mod options;
mod stats;
mod compress;
mod migration;

pub use db::{Db, Value, check::CheckOptions};
pub use table::Key;
pub use error::{Error, Result};
pub use options::{ColumnOptions, Options};
pub use migration::migrate;
pub use compress::CompressionType;
