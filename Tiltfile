
#k8s_yaml(local('helm template --set key1=val1,key2=val2 ./charts/main-chart'))
#watch_file('/charts/main-chart')

local_resource('pnpm', cmd='pnpm install', deps=['package.json', 'pnpm-lock.yaml'], labels=['pnpm'])

include('./k8s/base/helm/Tiltfile')

include('./apps/controller1/Tiltfile')
include('./apps/users/api/Tiltfile')
include('./apps/users/client/Tiltfile')
# include('./apps/infra/my-kube-controller/Tiltfile')
# include('./apps/infra/commdands/Tiltfile')

k8s_yaml(kustomize('k8s/base'))

load('ext://uibutton', 'cmd_button', 'location', 'text_input', 'bool_input')


cmd_button(name='NX',
        argv=['sh', '-c','pnpm nx $type --parallel --max-parallel=$cores --all $SKIP_CASHE --target=$TARGET'],
        text='NX',
        location=location.NAV,
        requires_confirmation=True,
        inputs=[
            text_input('type', placeholder='Enter your nx command type', default="affected"),
            text_input('TARGET', placeholder='Enter your nx command target', default="test"),
            bool_input('SKIP_CASHE', true_string='--skip-nx-cache', false_string=''),
            text_input('cores', placeholder='Enter value or --max-parallel', default="2"),
        ],
        icon_name='travel_explore')
    

# GOOS=linux GOARCH=amd64
