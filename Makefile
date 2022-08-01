GH_ORG=yurikrupnik
GCP_PROJECT:=$(gcloud config get-value project)
#NEW_SA_NAME=test-service-account-name
# local cluster start
.PHONY: all
all: build

##@ Docker
docker-rmi-all: ## Docker remove all images installed - TODO Fix not including
	images:=$(docker images -aq)
	docker rmi $images -f
docker-rm-all: ## Docker remove all images installed - TODO Fix not including
	docker rm $(docker ps -aq) -f
##@ General

.PHONY: help
help: ## Display this help.
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

.PHONY: set-gcp_project
set-gcp_project: ## Get using gcloud config current project
	GCP_PROJECT:=$(gcloud config get-value project)
	echo $GCP_PROJECT
	echo ${GCP_PROJECT}

	# failing
# Build Failed: kubernetes apply: error mapping monitoring.coreos.com/Alertmanager: no matches for kind "Alertmanager" in version "monitoring.coreos.com/v1"
	#helm template prometheus bitnami/kube-prometheus -n prometheus -f ./k8s/base/helm/values/prometheus-values.yaml > ./k8s/base/helm/manifests/grafana.yaml
	#kustomize build ./k8s/base/helm/manifests | kubectl apply -f -
k-b-a: compile-manifests
	kustomize build ./k8s/base/helm/manifests | kubectl apply -f -
k-b-d: compile-manifests
	kustomize build ./k8s/base/helm/manifests | kubectl delete -f -
# | kubectl apply -f -
up:
	ctlptl create registry ctlptl-registry --port=5005

	-#kind create cluster --name test-env --image kindest/node:v1.21.1 --config local-cluster/cluster.yaml
#	-nvm install node
#	tilt up
#	make tilt
down:
	-tilt down
	kind delete cluster --name test-env
# local cluster end


##@ Kubectl - shorthand k locally
kubectl-set-context:
	kubectl cluster-info --context kind-kind

##@ Helm
.PHOXY: compile-manifests-consul
compile-manifests-consul: ## Compile manifests from helm consul template to a single file.
	helm template consul hashicorp/consul -n consul -f ./k8s/base/helm/values/consul-values.yaml > ./k8s/base/helm/manifests/consul.yaml

.PHOXY: compile-manifests-grafana
compile-manifests-grafana: ## Compile manifests from helm grafana template to a single file.
	helm template grafana bitnami/grafana -n grafana -f ./k8s/base/helm/values/grafana-values.yaml > ./k8s/base/helm/manifests/grafana.yaml

.PHOXY: compile-manifests
compile-manifests: ## Compile manifests from helm template to a single file
	#make compile-manifests-grafana
	#helm template consul hashicorp/consul -n consul -f ./k8s/base/helm/values/consul-values.yaml > ./k8s/base/helm/manifests/consul.yaml
	#helm template grafana bitnami/grafana -n grafana -f ./k8s/base/helm/values/grafana-values.yaml > ./k8s/base/helm/manifests/grafana.yaml
	#helm template redis bitnami/redis -n redis -f ./k8s/base/helm/values/redis-values.yaml > ./k8s/base/helm/manifests/redis.yaml
	helm template vault hashicorp/vault -n vault -f ./k8s/base/helm/values/vault-values.yaml > ./k8s/base/helm/manifests/vault.yaml
create-repo: ## Create helm local repo
	#helm package charts/main-charts

helm-install: ## Install helm local chart repo
	helm install --name main-charts charts/main-charts

helm-lint: ## Run linter for local chart
	helm lint charts/main-charts

helm-dry-run: ## Run install dry run
	helm install --dry-run --debug charts/main-charts/ --generate-name --values services/my-nginx/values.yaml

helm-install:
	helm install example1 charts/main-charts --values values/values.yaml

helm-uninstall:
	helm uninstall example1
# helm end

define get-secret
$(shell gcloud secrets versions access latest --secret=MONGO_URI --project=mussia14)
endef

##@ Tilt
tilt-up: ## Tilt up in local cluster
	tilt up

tilt-down: ## Tilt down in local cluster
	tilt down

##@ Argo cd
get-argocd-secret-yaml: ## Argo cd get get init admin password
	kubectl get secret argocd-initial-admin-secret -n argocd -o jsonpath='{.data.password}' | base64 -d
# create service account
##@ GCP

csa: ## Secret stuff
		SA="${NEW_SA_NAME}@${PROJECT_ID}.iam.gserviceaccount.com"
		gcloud iam service-accounts create $NEW_SA_NAME --project $PROJECT_ID
		# enable cloud API
    SERVICE="sqladmin.googleapis.com"
#    gcloud services enable $SERVICE --project $GCP_PROJECT
    # grant access to cloud API
    ROLE="roles/cloudsql.admin"
    gcloud projects add-iam-policy-binding --role="$ROLE" $PROJECT_ID --member "serviceAccount:$SA"

    # create service account keyfile
#    gcloud iam service-accounts keys create creds.json --project $PROJECT_ID --iam-account $SA
#		kubectl create secret generic gcp-creds -n crossplane-system --from-file=creds=./creds.json

minikube-gcp: ## Create GCP SA with sql admin role
	SA="${NEW_SA_NAME}@${PROJECT_ID}.iam.gserviceaccount.com"
	gcloud iam service-accounts create $NEW_SA_NAME --project $PROJECT_ID
	ROLE="roles/cloudsql.admin"
#  gcloud projects add-iam-policy-binding --role="$ROLE" $PROJECT_ID --member "serviceAccount:$SA"
#  gcloud iam service-accounts keys create creds.json --project $PROJECT_ID --iam-account $SA

#	kubectl create namespace crossplane-system
#	helm repo add crossplane-stable https://charts.crossplane.io/stable
#	helm repo update
#	helm install crossplane --namespace crossplane-system crossplane-stable/crossplane
	#kubectl create secret generic gcp-creds -n crossplane-system --from-file=creds=./creds.json



	# Patches
# does create a secret but does not work with pulling images - todo check
#  kubectl create secret docker-registry docker-registry-key --docker-server=europe-west1-docker.pkg.dev --docker-username=oauth3accesstoken --docker-password="$(gcloud auth print-access-token)" --docker-email=krupnik.yuri@gmail.com
	#kubectl patch serviceaccount default -p '{"imagePullSecrets": [{"name": "docker-registry-key"}]}'
	#kubectl patch serviceaccount default -p '{"imagePullSecrets": [{"name": "gcp-creds"}]}'
	#kubectl patch serviceaccount default -p '{"imagePullSecrets": [{"name": "gcr-access-token"}]}'
	# settings Kubecost
	#kubectl create namespace kubecost
#  helm repo add kubecost https://kubecost.github.io/cost-analyzer/
#  helm install kubecost kubecost/cost-analyzer --namespace kubecost --set kubecostToken="a3J1cG5pay55dXJpQGdtYWlsLmNvbQ==xm343yadf98"
	#kubectl port-forward --namespace kubecost deployment/kubecost-cost-analyzer 9090

	kustomize build k8s/base | kubectl apply -f -

# in cluster
	#docker login -u oauth2accesstoken --password=ya29.A0ARrdaM-9yadNtYAEScIdz0RxjbfSkw79tAg6ZmIlfzkQe1PXnFUcyKsLNqtsw7DgrO-zLuRkkLLWGmhVPSTMqTqncYNLH9kXV8keW1hZ8dt04IOY17jnxJJyCGLSkhU0DpzyxyVSB7Zb2V-Jb5wgHww08sLEGM0ugwi1sQ https://europe-west1-docker.pkg.dev
	#cat .docker/config.json | base64
#	 update then the secrets file with dockerfonfig base64

##@ NX
affected-graph: ## Run local nx affected dep graph
	yarn nx affected:dep-graph
# NX end
