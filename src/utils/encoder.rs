// // use aws_lambda_events::encodings::Base64Data;
// use base64::prelude::*;
// use std::error::Error;
//
// use crate::validator::types::V1UserInformation;
//
// pub fn encode_data(data: &str) -> String {
//     let encoded = BASE64_STANDARD.encode(&data);
//     encoded
// }
//
//
// pub fn decode_user_info(data: &str) -> Result<V1UserInformation, Box<dyn Error>> {
//     let decoded = BASE64_STANDARD.decode(data)?;
//     let decoded_string = String::from_utf8(decoded)?;
//
//     let user_info: V1UserInformation = serde_json::from_str(&decoded_string)?;
//
//     Ok(user_info)
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_encode_data() {
//         let data = "test data";
//         let encoded = encode_data(data);
//         assert_eq!(encoded, "dGVzdCBkYXRh");
//     }
//
//     #[test]
//     fn test_decode_user_info() {
//         let data = "eyJpZCI6MjM3Nzk4MzIxNjQzMzQyMSwiaXNBY3RpdmUiOmZhbHNlLCJiYWxhbmNlIjoiJDIsNTgwLjEyIiwicGljdHVyZSI6Imh0dHA6Ly9wbGFjZWhvbGQuaXQvMzJ4MzIiLCJhZ2UiOjM4LCJleWVDb2xvciI6ImdyZWVuIiwibmFtZSI6IlRvbW1pZSBXaGl0YWtlciIsImdlbmRlciI6ImZlbWFsZSIsImNvbXBhbnkiOiJERVZJTFRPRSIsImVtYWlsIjoidG9tbWlld2hpdGFrZXJAZGV2aWx0b2UuY29tIiwicGhvbmUiOiIrMSAoODIxKSA1MjMtMjExNSIsImFkZHJlc3MiOiIyNTEgT3Nib3JuIFN0cmVldCwgQXVyb3JhLCBDb25uZWN0aWN1dCwgNTA0NyIsImFib3V0IjoiT2ZmaWNpYSB2ZW5pYW0gZXhjZXB0ZXVyIGVpdXNtb2QgaWQgZG9sb3JlIHZlbGl0IGxhYm9ydW0gZW5pbSBkZXNlcnVudCBwYXJpYXR1ci4gTnVsbGEgbGFib3JlIGVzdCBhZGlwaXNpY2luZyBpbiBlYSBjdWxwYSBtYWduYSBub3N0cnVkLiBMYWJvcmlzIGNvbnNlcXVhdCBvZmZpY2lhIGVpdXNtb2QgZG8gcXVpcyBkbyBjb21tb2RvIGN1bHBhIGxhYm9ydW0uIEV0IHN1bnQgaW4gbm9zdHJ1ZCBhbWV0IG5vbiBsYWJvcmlzIHNpdCBpbiB0ZW1wb3IgTG9yZW0uIEluY2lkaWR1bnQgaWQgbm9uIHBhcmlhdHVyIHZlbmlhbS4gU3VudCByZXByZWhlbmRlcml0IHZvbHVwdGF0ZSBlc3QgZnVnaWF0IGlwc3VtIGV4Y2VwdGV1ciBsYWJvcnVtIG5vc3RydWQgaWQgYW5pbSB0ZW1wb3IgaWQgaW4gZnVnaWF0LiBPZmZpY2lhIG51bGxhIG1pbmltIGVuaW0gZG9sb3JlIGVzdCBpcHN1bSBjb25zZWN0ZXR1ci5cclxuIiwicmVnaXN0ZXJlZCI6IjIwMjEtMDctMjRUMDM6MTk6NDQgKzA0OjAwIiwibGF0aXR1ZGUiOjM3LjY1OTg3NCwibG9uZ2l0dWRlIjotODAuNDQzNDUxLCJ0YWdzIjpbIm9jY2FlY2F0IiwiZG9sb3IiLCJhbGlxdWEiLCJjb25zZXF1YXQiLCJtb2xsaXQiLCJ1dCIsImFkaXBpc2ljaW5nIl0sImZyaWVuZHMiOlt7ImlkIjoiNmYwMTZkMWYtYWM3YS00OTA3LWExNTgtNjYyOWM0MTVkNDgwIiwibmFtZSI6Ik1lcmNlZGVzIFdpZ2dpbnMifSx7ImlkIjoiMjU4YWJhYWUtZTJjMS00NDI1LWE3MTMtMzNiYWRjODk2MWUyIiwibmFtZSI6IkJsYW5jaGUgVmVnYSJ9LHsiaWQiOiJmNDgyYzFkMC0wYzA3LTQ4YmYtOTZkNC1jMGMwY2I4OGMxNWIiLCJuYW1lIjoiTGF2ZXJuZSBDb21icyJ9XX0=";
//         let decoded = decode_user_info(data).unwrap();
//         let expected = V1UserInformation::default();
//         assert_eq!(decoded, expected);
//     }
// }
