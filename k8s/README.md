Make sure that the `newsletter` namespace exists and following secrets are
added before applying the recipes.

All services use the same AWS user with

```bash
k create secret generic aws \
    --from-literal=key=xxx \
    --from-literal=secret=xxx \
    --from-literal=region=xxx \
    -n newsletter
```

and all services use the same docker hub access token with

```bash
k create secret docker-registry regcred \
    --docker-username=porkbrain \
    --docker-password=xxx \
    --docker-email=michael@porkbrain.com \
    -n newsletter
```

The settings specific to each service should be put into a .env file and applied
with

```bash
k create secret generic service-name \
    --from-env-file=.env.service-name \
    -n newsletter
```

Some secrets are created from files with

```bash
k create secret generic gcp-ocr \
  --from-file=json=./.env.google.json \
  -n newsletter
```

