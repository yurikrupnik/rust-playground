/*
Copyright 2022.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

package controllers

import (
	"context"
	"github.com/go-logr/logr"
	yuriv1alpha1 "github.com/yurikrupnik/yuri-operator/api/v1alpha1"
	"k8s.io/apimachinery/pkg/runtime"
	ctrl "sigs.k8s.io/controller-runtime"
	"sigs.k8s.io/controller-runtime/pkg/client"
	"sigs.k8s.io/controller-runtime/pkg/log"
)

// ContainerInjectorReconciler reconciles a ContainerInjector object
type ContainerInjectorReconciler struct {
	client.Client
	Log    logr.Logger
	Scheme *runtime.Scheme
}

//+kubebuilder:rbac:groups=yuri.dev.yurikrupnik,resources=containerinjectors,verbs=get;list;watch;create;update;patch;delete
//+kubebuilder:rbac:groups=yuri.dev.yurikrupnik,resources=containerinjectors/status,verbs=get;update;patch
//+kubebuilder:rbac:groups=yuri.dev.yurikrupnik,resources=containerinjectors/finalizers,verbs=update

// Reconcile is part of the main kubernetes reconciliation loop which aims to
// move the current state of the cluster closer to the desired state.
// TODO(user): Modify the Reconcile function to compare the state specified by
// the ContainerInjector object against the actual cluster state, and then
// perform operations to make the cluster state reflect the state specified by
// the user.
//
// For more details, check Reconcile and its Result here:
// - https://pkg.go.dev/sigs.k8s.io/controller-runtime@v0.12.2/pkg/reconcile
func (r *ContainerInjectorReconciler) Reconcile(ctx context.Context, req ctrl.Request) (ctrl.Result, error) {
	var containerInjector yuriv1alpha1.ContainerInjector
	//ctx := context.Background()
	l := log.FromContext(ctx)
	//log := r.Log.WithValues("container-injector", req.NamespacedName)
	//log.Info("hello from container injector ctrl", "name", req.NamespacedName)
	ll := l.WithValues("container-injector", req.NamespacedName)
	//ll.Info("Aris is god \n")
	//ll.Info("Aris is god1\n")
	//println("Aris is god3\n")
	println("Namespace: %s\n", req.Namespace)
	println("name %s\n", req.Name)
	println("ctx %s\n", ctx)
	if err := r.Get(ctx, req.NamespacedName, &containerInjector); err != nil {
		ll.Info("Error getting object\n", "name", req.NamespacedName)
		return ctrl.Result{}, client.IgnoreNotFound(err)
	}

	if err := r.Status().Update(ctx, &containerInjector); err != nil {
		ll.Info("Error updating status\n", "name", req.NamespacedName)
		return ctrl.Result{}, client.IgnoreNotFound(err)
	}

	//if yuriv1alpha1.ContainerInjectorStatus{}

	//if containerInjector
	//println("ctx %s", r.List())
	//r.Get()
	//println("NamespacedName %T", req.NamespacedName)
	//println("name %s", yuriv1alpha1.ContainerInjectorSpec{
	//	Foo:     "",
	//	Alerts:  false,
	//	Alertss: false,
	//	Images:  nil,
	//})
	// TODO(user): your logic here

	return ctrl.Result{}, nil
}

// SetupWithManager sets up the controller with the Manager.
func (r *ContainerInjectorReconciler) SetupWithManager(mgr ctrl.Manager) error {
	return ctrl.NewControllerManagedBy(mgr).
		For(&yuriv1alpha1.ContainerInjector{}).
		Complete(r)
}
