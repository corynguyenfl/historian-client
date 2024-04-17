#!/bin/sh

# SPDX-FileCopyrightText: 2024 Open Energy Solutions Inc
#
# SPDX-License-Identifier: Apache-2.0

find . -name "*.rs" | xargs reuse addheader -l Apache-2.0 -y 2024 -c "Open Energy Solutions Inc" --copyright-style spdx

