docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate \
    -i /local/wiremock_openapi_spec.json \
    -g rust \
    -o /local/wiremock-client \
    --skip-validate-spec \
    --additional-properties=packageName=wiremock-client,supportAsync=true