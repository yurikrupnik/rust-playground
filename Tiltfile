
#k8s_yaml(local('helm template --set key1=val1,key2=val2 ./charts/main-chart'))
#watch_file('/charts/main-chart')

# local_resource(
#   "make k-b-a",
#   # cmd="make k-b-a",
#   cmd="ls",
#   allow_parallel = True,
#   trigger_mode=TRIGGER_MODE_MANUAL,
#   # only=["/k8s/base/helm/values/"],
#   deps=["/k8s/base/helm/values/"],
#   labels=["makefile", "helm", "manual"],
# )
# local_resource(
#   "make k-b-d",
#   cmd="ls",
#   # cmd="make k-b-d",
#   allow_parallel = True,
#   # trigger_mode=TRIGGER_MODE_MANUAL,
#   # TRIGGER_MODE_AUTO
#   # only=["/k8s/base/helm/values/"],
#   deps=["/k8s/base/helm/values/"],
#   labels=["makefile", "helm", "manual"],
# )

local_resource('pnpm', cmd='pnpm install', deps=['package.json', 'pnpm-lock.yaml'], labels=['pnpm'])

# include('./k8s/base/helm/Tiltfile')

include('./apps/controller1/Tiltfile')
include('./apps/users/api/Tiltfile')
include('./apps/users/client/Tiltfile')
# include('./apps/infra/my-kube-controller/Tiltfile')
# include('./apps/infra/commdands/Tiltfile')

k8s_yaml(kustomize('k8s/base'))
# k8s_yaml(kustomize('k8s/base/helm/manifests'))
# load('ext://uibutton', 'cmd_button', 'location', 'text_input')
# cmd_button('letters:pnpm install',
#     argv=['pnpm install'],
#     resource='dsclient',
#     icon_name='cloud_download',
#     text='pnpm install',
#     requires_confirmation=True
# )

# cmd_button(name='lint',
#            resource='frontend',
#            argv=['yarn', 'run', 'eslint', '.'])
# k8s_resource("users-api", port_forwards="5001:8080")
# ports to container port that runs as container env var - both ways
# k8s_resource("users-api", port_forwards="5001:8080")
# k8s_resource(workload='users-api', port_forwards="5001:8080")


# GOOS=linux GOARCH=amd64
