aws dynamodb create-table \
  --cli-input-json '
{
  "TableName": "StatsDB-Test",
  "AttributeDefinitions": [
    {
      "AttributeName": "GSI1-PK",
      "AttributeType": "S"
    },
    {
      "AttributeName": "GSI1-SK",
      "AttributeType": "S"
    },
    {
      "AttributeName": "GSI2-PK",
      "AttributeType": "S"
    },
    {
      "AttributeName": "GSI2-SK",
      "AttributeType": "S"
    },
    {
      "AttributeName": "LSI1-SK",
      "AttributeType": "S"
    },
    {
      "AttributeName": "PK",
      "AttributeType": "S"
    },
    {
      "AttributeName": "SK",
      "AttributeType": "S"
    }
  ],
  "KeySchema": [
    {
      "AttributeName": "PK",
      "KeyType": "HASH"
    },
    {
      "AttributeName": "SK",
      "KeyType": "RANGE"
    }
  ],
  "LocalSecondaryIndexes": [
    {
      "IndexName": "LSI1",
      "KeySchema": [
        {
          "AttributeName": "PK",
          "KeyType": "HASH"
        },
        {
          "AttributeName": "LSI1-SK",
          "KeyType": "RANGE"
        }
      ],
      "Projection": {
        "ProjectionType": "ALL"
      }
    }
  ],
  "GlobalSecondaryIndexes": [
    {
      "IndexName": "GSI2",
      "KeySchema": [
        {
          "AttributeName": "GSI2-PK",
          "KeyType": "HASH"
        },
        {
          "AttributeName": "GSI2-SK",
          "KeyType": "RANGE"
        }
      ],
      "Projection": {
        "ProjectionType": "ALL"
      }
    },
    {
      "IndexName": "GSI1",
      "KeySchema": [
        {
          "AttributeName": "GSI1-PK",
          "KeyType": "HASH"
        },
        {
          "AttributeName": "GSI1-SK",
          "KeyType": "RANGE"
        }
      ],
      "Projection": {
        "ProjectionType": "ALL"
      }
    }
  ],
  "BillingMode": "PAY_PER_REQUEST",
  "TableClass": "STANDARD"
}
'

