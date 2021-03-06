// Copyright 2016 The Cartographer Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use error_chain::error_chain;
use std::io;

error_chain! {
    foreign_links {
        Io(io::Error);
    }

    errors {
        InvalidInput(msg: String) {
            description("The input is not supported or invalid")
            display("{}", msg)
        }

        InvalidVersion(version: i32) {
            description("invalid octree version on disk")
            display(
            "Octree in this directory has a version of {}, the only supported version is {}. \
            Try running upgrade_octree on this data to get it to the current version. \
            The viewer might eventually be backwards compatible, but for now only \
            the currently created version is supported.", version, crate::CURRENT_VERSION)
        }

        NodeNotFound {
            description("The node does not exist.")
        }

        Channel(msg: String) {
            description("The current channel failed an operation")
            display("{}", msg)
        }

    }
}
