/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StateResponseLineItem {
    line_item_expense_fields: Vec<StateResponseLineItemExpenseField>,
}