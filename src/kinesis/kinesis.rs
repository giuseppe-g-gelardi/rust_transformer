use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KinesisRecord {
    pub kinesis: Kinesis,
    pub event_source: String,
    pub event_version: String,
    #[serde(rename = "eventID")]
    pub event_id: String,
    pub event_name: String,
    pub invoke_identity_arn: String,
    pub aws_region: String,
    #[serde(rename = "eventSourceARN")]
    pub event_source_arn: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kinesis {
    pub data: String,
    pub partition_key: String,
    pub kinesis_schema_version: String,
    pub sequence_number: String,
    pub approximate_arrival_timestamp: f64,
}

#[derive(Deserialize, Debug)]
pub struct KinesisEvent {
    #[serde(rename = "Records")]
    pub records: Vec<KinesisRecord>,
}

impl KinesisRecord {
    #[cfg(test)]
    pub fn default() -> Self {
        KinesisRecord {
            kinesis: Kinesis {
                data: "eyJpZCI6MjM3Nzk4MzIxNjQzMzQyMSwiaXNBY3RpdmUiOmZhbHNlLCJiYWxhbmNlIjoiJDIsNTgwLjEyIiwicGljdHVyZSI6Imh0dHA6Ly9wbGFjZWhvbGQuaXQvMzJ4MzIiLCJhZ2UiOjM4LCJleWVDb2xvciI6ImdyZWVuIiwibmFtZSI6IlRvbW1pZSBXaGl0YWtlciIsImdlbmRlciI6ImZlbWFsZSIsImNvbXBhbnkiOiJERVZJTFRPRSIsImVtYWlsIjoidG9tbWlld2hpdGFrZXJAZGV2aWx0b2UuY29tIiwicGhvbmUiOiIrMSAoODIxKSA1MjMtMjExNSIsImFkZHJlc3MiOiIyNTEgT3Nib3JuIFN0cmVldCwgQXVyb3JhLCBDb25uZWN0aWN1dCwgNTA0NyIsImFib3V0IjoiT2ZmaWNpYSB2ZW5pYW0gZXhjZXB0ZXVyIGVpdXNtb2QgaWQgZG9sb3JlIHZlbGl0IGxhYm9ydW0gZW5pbSBkZXNlcnVudCBwYXJpYXR1ci4gTnVsbGEgbGFib3JlIGVzdCBhZGlwaXNpY2luZyBpbiBlYSBjdWxwYSBtYWduYSBub3N0cnVkLiBMYWJvcmlzIGNvbnNlcXVhdCBvZmZpY2lhIGVpdXNtb2QgZG8gcXVpcyBkbyBjb21tb2RvIGN1bHBhIGxhYm9ydW0uIEV0IHN1bnQgaW4gbm9zdHJ1ZCBhbWV0IG5vbiBsYWJvcmlzIHNpdCBpbiB0ZW1wb3IgTG9yZW0uIEluY2lkaWR1bnQgaWQgbm9uIHBhcmlhdHVyIHZlbmlhbS4gU3VudCByZXByZWhlbmRlcml0IHZvbHVwdGF0ZSBlc3QgZnVnaWF0IGlwc3VtIGV4Y2VwdGV1ciBsYWJvcnVtIG5vc3RydWQgaWQgYW5pbSB0ZW1wb3IgaWQgaW4gZnVnaWF0LiBPZmZpY2lhIG51bGxhIG1pbmltIGVuaW0gZG9sb3JlIGVzdCBpcHN1bSBjb25zZWN0ZXR1ci5cclxuIiwicmVnaXN0ZXJlZCI6IjIwMjEtMDctMjRUMDM6MTk6NDQgKzA0OjAwIiwibGF0aXR1ZGUiOjM3LjY1OTg3NCwibG9uZ2l0dWRlIjotODAuNDQzNDUxLCJ0YWdzIjpbIm9jY2FlY2F0IiwiZG9sb3IiLCJhbGlxdWEiLCJjb25zZXF1YXQiLCJtb2xsaXQiLCJ1dCIsImFkaXBpc2ljaW5nIl0sImZyaWVuZHMiOlt7ImlkIjoiNmYwMTZkMWYtYWM3YS00OTA3LWExNTgtNjYyOWM0MTVkNDgwIiwibmFtZSI6Ik1lcmNlZGVzIFdpZ2dpbnMifSx7ImlkIjoiMjU4YWJhYWUtZTJjMS00NDI1LWE3MTMtMzNiYWRjODk2MWUyIiwibmFtZSI6IkJsYW5jaGUgVmVnYSJ9LHsiaWQiOiJmNDgyYzFkMC0wYzA3LTQ4YmYtOTZkNC1jMGMwY2I4OGMxNWIiLCJuYW1lIjoiTGF2ZXJuZSBDb21icyJ9XX0=".to_string(),

                partition_key: "partitionKey-1".to_string(),
                kinesis_schema_version: "1.0".to_string(),
                sequence_number: "2000000000000001".to_string(),
                approximate_arrival_timestamp: 1510261770.0,
            },
            event_source: "aws:kinesis".to_string(),
            event_version: "1.0".to_string(),
            event_id: "shardId-000000000000:2000000000000001".to_string(),
            event_name: "aws:kinesis:record".to_string(),
            invoke_identity_arn: "arn:aws:iam::EXAMPLE".to_string(),
            aws_region: "us-east-1".to_string(),
            event_source_arn: "arn:aws:kinesis:EXAMPLE".to_string(),
        }
    }

    #[cfg(test)]
    pub fn default_output() -> Self {
        KinesisRecord {
            kinesis: Kinesis {
                data: "eyJpZCI6MjM3Nzk4MzIxNjQzMzQyMSwiYWNjb3VudEluZm9ybWF0aW9uIjp7ImlzQWN0aXZlIjpmYWxzZSwicmVnaXN0ZXJlZCI6IjIwMjEtMDctMjRUMDM6MTk6NDQgKzA0OjAwIiwiYmFsYW5jZSI6IiQyLDU4MC4xMiJ9LCJ1c2VySW5mb3JtYXRpb24iOnsiZmlyc3ROYW1lIjoiVG9tbWllIiwibGFzdE5hbWUiOiJXaGl0YWtlciIsImFnZSI6MzgsImdlbmRlciI6ImZlbWFsZSIsImV5ZUNvbG9yIjoiZ3JlZW4iLCJwaWN0dXJlIjoiaHR0cDovL3BsYWNlaG9sZC5pdC8zMngzMiIsImNvbXBhbnkiOiJERVZJTFRPRSJ9LCJjb250YWN0SW5mb3JtYXRpb24iOnsiZW1haWwiOiJ0b21taWV3aGl0YWtlckBkZXZpbHRvZS5jb20iLCJwaG9uZSI6IisxICg4MjEpIDUyMy0yMTE1IiwiYWRkcmVzcyI6eyJzdHJlZXQiOiIyNTEgT3Nib3JuIFN0cmVldCIsImNpdHkiOiJBdXJvcmEiLCJzdGF0ZSI6IkNvbm5lY3RpY3V0IiwiemlwIjo1MDQ3fX0sInRhZ3MiOlsib2NjYWVjYXQiLCJkb2xvciIsImFsaXF1YSIsImNvbnNlcXVhdCIsIm1vbGxpdCIsInV0IiwiYWRpcGlzaWNpbmciXSwicHJvZmlsZSI6Ik9mZmljaWEgdmVuaWFtIGV4Y2VwdGV1ciBlaXVzbW9kIGlkIGRvbG9yZSB2ZWxpdCBsYWJvcnVtIGVuaW0gZGVzZXJ1bnQgcGFyaWF0dXIuIE51bGxhIGxhYm9yZSBlc3QgYWRpcGlzaWNpbmcgaW4gZWEgY3VscGEgbWFnbmEgbm9zdHJ1ZC4gTGFib3JpcyBjb25zZXF1YXQgb2ZmaWNpYSBlaXVzbW9kIGRvIHF1aXMgZG8gY29tbW9kbyBjdWxwYSBsYWJvcnVtLiBFdCBzdW50IGluIG5vc3RydWQgYW1ldCBub24gbGFib3JpcyBzaXQgaW4gdGVtcG9yIExvcmVtLiBJbmNpZGlkdW50IGlkIG5vbiBwYXJpYXR1ciB2ZW5pYW0uIFN1bnQgcmVwcmVoZW5kZXJpdCB2b2x1cHRhdGUgZXN0IGZ1Z2lhdCBpcHN1bSBleGNlcHRldXIgbGFib3J1bSBub3N0cnVkIGlkIGFuaW0gdGVtcG9yIGlkIGluIGZ1Z2lhdC4gT2ZmaWNpYSBudWxsYSBtaW5pbSBlbmltIGRvbG9yZSBlc3QgaXBzdW0gY29uc2VjdGV0dXIuXHJcbiJ9".to_string(),
                partition_key: "partitionKey-1".to_string(),
                kinesis_schema_version: "1.0".to_string(),
                sequence_number: "2000000000000001".to_string(),
                approximate_arrival_timestamp: 1510261770.0,
            },
            event_source: "aws:kinesis".to_string(),
            event_version: "1.0".to_string(),
            event_id: "shardId-000000000000:2000000000000001".to_string(),
            event_name: "aws:kinesis:record".to_string(),
            invoke_identity_arn: "arn:aws:iam::EXAMPLE".to_string(),
            aws_region: "us-east-1".to_string(),
            event_source_arn: "arn:aws:kinesis:EXAMPLE".to_string(),
        }
    }
}
