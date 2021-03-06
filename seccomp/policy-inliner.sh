#!/bin/bash
# Copyright (C) 2020 The Android Open Source Project
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

common_device="$1"
gpu_common="$2"
if ! [[ -f $common_device ]] || ! [[ -f $gpu_common ]]; then
  echo "usage: $0 /path/to/common_device.policy /path/to/gpu_common.policy <input.policy >output.policy"
  exit 1
fi

while IFS= read -r line
do
  if echo "$line" | egrep "@include[[:space:]]+/usr/share/policy/crosvm/common_device.policy" > /dev/null; then
    cat $common_device
    continue
  elif echo "$line" | egrep "@include[[:space:]]+/usr/share/policy/crosvm/gpu_common.policy" > /dev/null; then
    cat $gpu_common
    continue
  fi
  echo $line
done
