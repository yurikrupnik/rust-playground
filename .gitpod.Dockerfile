FROM gitpod/workspace-full:2022-05-08-14-31-53

# Install custom tools, runtime, etc.
RUN brew install kubectl kustomize ctlptl helm titl
RUN ctlptl create cluster k3d --registry=ctlptl-regist
