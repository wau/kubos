//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use super::*;
use serde_json::json;

#[test]
fn get_lock_status_default() {
    let mut mock = MockStream::default();

    let service = service_new!(mock);

    let query = r#"{
            lockStatus {
                positionStatus,
                positionType,
                time {
                    ms,
                    week
                },
                timeStatus,
                velocityStatus,
                velocityType
            }
        }"#;

    let expected = json!({
            "lockStatus": {
                "positionStatus": "INSUFFICIENT_OBSERVATIONS",
                "positionType": "NONE",
                "time": {
                    "ms": 0,
                    "week": 0
                },
                "timeStatus": "UNKNOWN",
                "velocityStatus": "INSUFFICIENT_OBSERVATIONS",
                "velocityType": "NONE"
            }
    });

    test!(service, query, expected);
}

#[test]
fn get_lock_status_good() {
    let mut mock = MockStream::default();

    mock.read.set_output(POSITION_LOG_NO_LOCK.to_vec());

    let service = service_new!(mock);

    let query = r#"{
            lockStatus {
                positionStatus,
                positionType,
                time {
                    ms,
                    week
                },
                timeStatus,
                velocityStatus,
                velocityType
            }
        }"#;

    let expected = json!({
            "lockStatus": {
                "positionStatus": "INSUFFICIENT_OBSERVATIONS",
                "positionType": "NONE",
                "time": {
                    "ms": 164195000,
                    "week": 3025
                },
                "timeStatus": "COARSE_STEERING",
                "velocityStatus": "INSUFFICIENT_OBSERVATIONS",
                "velocityType": "NONE"
            }
    });

    test!(service, query, expected);
}

#[test]
fn get_lock_status_nondefault() {
    let mut mock = MockStream::default();

    mock.read.set_output(POSITION_LOG_GOOD.to_vec());

    let service = service_new!(mock);

    let query = r#"{
            lockStatus {
                positionStatus,
                positionType,
                time {
                    ms,
                    week
                },
                timeStatus,
                velocityStatus,
                velocityType
            }
        }"#;

    let expected = json!({
            "lockStatus": {
                "positionStatus": "SOL_COMPUTED",
                "positionType": "PSRDIFF",
                "time": {
                    "ms": 164195000,
                    "week": 3025
                },
                "timeStatus": "FINE_STEERING",
                "velocityStatus": "SOL_COMPUTED",
                "velocityType": "PSRDIFF"
            }
    });

    test!(service, query, expected);
}
