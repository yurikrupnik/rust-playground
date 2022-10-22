set export
set shell := ["fish", "-c"]
browse := if os() == "linux" { "xdg-open" } else { "open" }
copy        := if os() == "linux" { "xsel -ib"} else { "pbcopy" }

argocd_port  := "30950"

default:
    @just --list --unsorted

# k3d cluster test - does not work (fails to start ingress) - same with docker kubernetes
ingress-1:
  k3d cluster create
  kubectx k3d-k3s-default
  kubectl create deployment web --image=gcr.io/google-samples/hello-app:1.0
  kubectl expose deployment web --type=ClusterIP --port=8080
  kubectl apply -f https://k8s.io/examples/service/networking/example-ingress.yaml
# kind cluster test - does not work, fails to load ingress
ingress-2:
  kind create cluster
  kubectx kind-kind
  kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/kind/deploy.yaml
  kubectl wait --namespace ingress-nginx \
    --for=condition=ready pod \
    --selector=app.kubernetes.io/component=controller \
    --timeout=90s
  kubectl create deployment web --image=gcr.io/google-samples/hello-app:1.0
  kubectl expose deployment web --type=ClusterIP --port=8080
  kubectl apply -f https://k8s.io/examples/service/networking/example-ingress.yaml
# minikube cluster test - works
ingress-3:
  minikube start
  kubectx minikube
  minikube addons enable ingress
  kubectl create deployment web --image=gcr.io/google-samples/hello-app:1.0
  kubectl expose deployment web --type=ClusterIP --port=8080
  kubectl apply -f https://k8s.io/examples/service/networking/example-ingress.yaml
  minikube tunnel
#  kubectl apply -f https://kind.sigs.k8s.io/examples/ingress/usage.yaml


# setup k3d cluster
setup-k3d:
  k3d cluster create k3d-cluster --k3s-arg "--disable=traefik@server:0"
# setup k3d cluster
destroy-k3d:
  k3d
# setup kind cluster
setup-kind:
  kind create cluster
# setup argocd
setup-argocd:
  kustomize build ./gitops/argocd | kubectl create -f -
# setup cert-manager, external-secrets, metallb
setup-apps:
  kubectl apply -f ./gitops/sealed-secret/app2.yaml #   is it cert-manager
# setup istio
setup-istio: setup-kind setup-argocd
  kubectl wait --for condition=Available=True --timeout=300s deployment/argocd-repo-server -n argocd
  @just setup-apps
  @sleep 50
  kubectl wait --for condition=Available=True --timeout=300s deployment/metallb-controller -n metallb-system
  kubectl apply -f ./gitops/metallb/config.yaml
#  istioctl install --set profile=default -y # change this to helm
#  kubectl label namespace default istio-injection=enabled
#  kubectl apply -f https://raw.githubusercontent.com/istio/istio/release-1.15/samples/bookinfo/platform/kube/bookinfo.yaml --wait
#  kubectl apply -f https://raw.githubusercontent.com/istio/istio/release-1.15/samples/bookinfo/networking/bookinfo-gateway.yaml
#  kubectl apply -f https://raw.githubusercontent.com/istio/istio/master/samples/addons/prometheus.yaml
#  kubectl apply -f https://raw.githubusercontent.com/istio/istio/master/samples/addons/grafana.yaml
#  kubectl apply -f https://raw.githubusercontent.com/istio/istio/master/samples/addons/kiali.yaml
#  kubectl apply -f https://raw.githubusercontent.com/istio/istio/master/samples/addons/jaeger.yaml
#  tilt up
#  kubectl -n argocd get secret argocd-initial-admin-secret -o jsonpath="{.data.password}" | base64 -d
#  helm repo add istio https://istio-release.storage.googleapis.com/charts
#  helm repo update
#  kubectl create namespace istio-system
#  helm install istio-base istio/base -n istio-system
#  helm install istiod istio/istiod -n istio-system --wait
#  kubectl create namespace istio-ingress
#  kubectl label namespace istio-ingress istio-injection=enabled
#  helm install istio-ingress istio/gateway -n istio-ingress --wait

setup-argocd-2:
  kustomize build ./gitops/argocd | kubectl create -f -
#  @kubectl wait --for condition=Available=True --timeout=300s deployment/argocd-server --namespace argocd
#  @kubectl patch svc argocd-server -n argocd -p '{"spec": {"type": "NodePort"}}'
#  @kubectl patch svc argocd-server -n argocd --type merge --type='json' -p='[{"op": "replace", "path": "/spec/ports/0/nodePort", "value": {{argocd_port}}}]'

launch_argo:
  @kubectl -n argocd get secret argocd-initial-admin-secret -o jsonpath="{.data.password}" | base64 -d | {{copy}}
  @nohup {{browse}} http://localhost:{{argocd_port}} >/dev/null 2>&1
# copy ArgoCD server secret to clipboard and launch browser without port forwarding
launch-argo: setup-kind setup-argocd
  @kubectl -n argocd get secret argocd-initial-admin-secret -o jsonpath="{.data.password}" | base64 -d | echo
#  @nohup {{browse}} http://localhost:{{argocd_port}} >/dev/null 2>&1 &
#  @just setup-cert-manager
#  kubectl -n argocd wait --for condition=established --timeout=60s crd/applications.argoproj.io
#  kubectl wait --for condition=Available=True --timeout=300s deployment/argocd-server -n argocd
#  kubectl create namespace argocd
#  kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml
# #!/usr/bin/env bash
#  set -euo pipefail
#  echo "creating kind cluster - {{cluster-name}}"
#  kubectl wait --for condition=Availible=True --timeout=300s deployment/ds --namespace argocd
# test stuff
test:
  echo testing
# shit stuff
shit:
  echo shitting
alias b := test
alias s := shit
