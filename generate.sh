#!/bin/bash

if [ ! -e openapi-generator-cli.jar ]; then
  curl https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/7.1.0/openapi-generator-cli-7.1.0.jar -o openapi-generator-cli.jar
fi

java -jar openapi-generator-cli.jar generate -i ../saasus-api/modules/v1/apilog/controller/apilogapi/apilogapi.yml -g rust -o ./generated/apilog --additional-properties=enumNameSuffix=Enum,library=hyper,packageName=apilog
java -jar openapi-generator-cli.jar generate -i ../saasus-api/modules/v1/auth/controller/authapi/authapi.yml -g rust -o ./generated/auth --additional-properties=enumNameSuffix=Enum,library=hyper,packageName=auth
java -jar openapi-generator-cli.jar generate -i ../saasus-api/modules/v1/awsmarketplace/controller/awsmarketplaceapi/awsmarketplaceapi.yml -g rust -o ./generated/awsmarketplace --additional-properties=enumNameSuffix=Enum,library=hyper,packageName=awsmarketplace
java -jar openapi-generator-cli.jar generate -i ../saasus-api/modules/v1/billing/controller/billingapi/billingapi.yml -g rust -o ./generated/billing --additional-properties=enumNameSuffix=Enum,library=hyper,packageName=billing
java -jar openapi-generator-cli.jar generate -i ../saasus-api/modules/v1/communication/controller/communicationapi/communicationapi.yml -g rust -o ./generated/communication --additional-properties=enumNameSuffix=Enum,library=hyper,packageName=communication
java -jar openapi-generator-cli.jar generate -i ../saasus-api/modules/v1/integration/controller/integrationapi/integration.yml -g rust -o ./generated/integration --additional-properties=enumNameSuffix=Enum,library=hyper,packageName=integration
java -jar openapi-generator-cli.jar generate -i ../saasus-api/modules/v1/pricing/controller/pricingapi/pricingapi.yml -g rust -o ./generated/pricing --additional-properties=enumNameSuffix=Enum,library=hyper,packageName=pricing
