#default shell arguments
#CFLAGS = -c -g -D $(OPT1) -D $(OPT2)
cluster-type=kind

GH_ORG=yurikrupnik
GCP_PROJECT:=$(gcloud config get-value project)

.PHONY: help
help: ## Display this help.
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Metallb
install-metallb-crd: ## Install Metallb CRDs
	kubectl apply -f https://raw.githubusercontent.com/metallb/metallb/v0.13.6/config/manifests/metallb-native.yaml
install-metallb: ## Install Metallb configs
	kubectl apply -f ./metallb/config.yaml
install-cert-manager:
	kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.9.1/cert-manager.yaml
install-argocd:
	kustomize build ./argocd  | kubectl create -f -
#	kubectl create namespace argocd
#  kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml

##@ kubernetes
create-cluster: ## Init kubernetes local cluster - default kind (k3s, minikube)
	#ctlptl create cluster $(cluster-type)
	ctlptl create cluster kind --registry=ctlptl-registry
	#kind create cluster --config cluster.yaml
delete-cluster: ## Destroy kubernetes local cluster
	#kind delete cluster
	ctlptl delete cluster kind

local-up:
	$(MAKE) create-cluster
	$(MAKE) install-metallb-crd
	sleep 50
	$(MAKE) install-metallb
	istioctl install --set profile=demo -y
	#$(MAKE) install-argocd
#	kubectl create namespace istio-system
#	helm install istio-base istio/base -n istio-system
#	helm install istiod istio/istiod -n istio-system
#	kubectl create namespace istio-ingress
#	kubectl label namespace istio-ingress istio-injection=enabled
#	helm install istio-ingress istio/gateway -n istio-ingress
	kubectl label namespace default istio-injection=enabled
	kubectl apply -f https://raw.githubusercontent.com/istio/istio/release-1.15/samples/bookinfo/platform/kube/bookinfo.yaml --wait
	kubectl apply -f https://raw.githubusercontent.com/istio/istio/release-1.15/samples/bookinfo/networking/bookinfo-gateway.yaml
	# prometheus
	kubectl apply -f https://raw.githubusercontent.com/istio/istio/master/samples/addons/prometheus.yaml
	# grafana
	kubectl apply -f https://raw.githubusercontent.com/istio/istio/master/samples/addons/grafana.yaml
	# kiali
	kubectl apply -f https://raw.githubusercontent.com/istio/istio/master/samples/addons/kiali.yaml
	# jaeger
	kubectl apply -f https://raw.githubusercontent.com/istio/istio/master/samples/addons/jaeger.yaml
#	export INGRESS_HOST=$(kubectl -n istio-system get service istio-ingressgateway -o jsonpath='{.status.loadBalancer.ingress[0].ip}')
#  export INGRESS_PORT=$(kubectl -n istio-system get service istio-ingressgateway -o jsonpath='{.spec.ports[?(@.name=="http2")].port}')
#  export SECURE_INGRESS_PORT=$(kubectl -n istio-system get service istio-ingressgateway -o jsonpath='{.spec.ports[?(@.name=="https")].port}')
test-traffic-app:
	watch -n 1 "curl 'http://localhost:62210/productpage'";
test-traffic-client:
	watch -n 1 "curl 'http://localhost:62210'"
test-traffic-api:
	watch -n 1 "curl 'http://localhost:62210/api/projects'"
	#for i in $(seq 1 300); do curl -s -o /dev/null "http://localhost:52908/productpage"; done
	#for i in $(seq 1 100); do echo hello; done
	#for i in $(seq 1 100); do curl -s -o /dev/null "http://172.24.0.10:80/productpage"; done

#local-down:
#	ctlptl delete cluster $(cluster-type)
#NEW_SA_NAME=test-service-account-name
# local cluster start
local-cluster:
	-k3d cluster create local-cluster --servers 1 --agents 3 --no-lb --subnet 172.25.0.0/16
	docker network ls
	cidr_block=$(docker network inspect k3d-local-cluster | jq '.[0].IPAM.Config[0].Subnet' |tr -d '"')
	echo $cidr_block
	ingress_range=172.25.255.0-172.25.255.254

delete-local-cluster:
	-k3d cluster delete local-cluster

sta:
	cidr_block=$(docker network inspect k3d-local-cluster | jq '.[0].IPAM.Config[0].Subnet' |tr -d '"')
	echo $cidr_block
	sipcalc ip_range

deploy:
	kubectl create deployment nginx --image=nginx
	kubectl expose deployment nginx --port=80 --type=LoadBalancer
