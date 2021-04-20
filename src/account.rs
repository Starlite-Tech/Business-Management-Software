/*

Created:0.0.1
updated:0.0.1

description:
  Contains Account Information
 */

//struct for account type
pub struct AccountType;
impl AccountType{
    pub const DEFAULT: i32 = 0;
    pub const ADMIN  : i32 = 1;
    pub const OWNER  : i32 = 2;
}