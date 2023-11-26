openapi-generator generate -i ../saasus-api/modules/v1/apilog/controller/apilogapi/apilogapi.yml -g rust -o ./generated/apilog --additional-properties=library=hyper,packageName=apilog
openapi-generator generate -i ../saasus-api/modules/v1/auth/controller/authapi/authapi.yml -g rust -o ./generated/auth --additional-properties=library=hyper,packageName=auth
openapi-generator generate -i ../saasus-api/modules/v1/awsmarketplace/controller/awsmarketplaceapi/awsmarketplaceapi.yml -g rust -o ./generated/awsmarketplace --additional-properties=library=hyper,packageName=awsmarketplace
openapi-generator generate -i ../saasus-api/modules/v1/billing/controller/billingapi/billingapi.yml -g rust -o ./generated/billing --additional-properties=library=hyper,packageName=billing
openapi-generator generate -i ../saasus-api/modules/v1/communication/controller/communicationapi/communicationapi.yml -g rust -o ./generated/communication --additional-properties=library=hyper,packageName=communication
openapi-generator generate -i ../saasus-api/modules/v1/integration/controller/integrationapi/integration.yml -g rust -o ./generated/integration --additional-properties=library=hyper,packageName=integration
openapi-generator generate -i ../saasus-api/modules/v1/pricing/controller/pricingapi/pricingapi.yml -g rust -o ./generated/pricing --additional-properties=library=hyper,packageName=pricing
