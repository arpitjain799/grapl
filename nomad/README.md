# Nomad

Nomad is our container orchestration service of choice.

## Adding a new service to Nomad

1. Define build steps within a Dockerfile, preferably one of the existing ones.
2. Add your service to `docker-bake.yml`; make sure to add it to the proper
   group(s) as well.
3. In the nomad file define a service-specific tag variable
4. In `pulumi/grapl/__main__.py`, add a new variable to the nomad job for the
   tag
