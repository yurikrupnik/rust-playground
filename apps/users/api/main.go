package main

import (
	"fmt"
	"github.com/gofiber/fiber/v2"

	"github.com/gofiber/fiber/v2/middleware/cors"
	"github.com/gofiber/fiber/v2/middleware/logger"
	"github.com/gofiber/fiber/v2/middleware/monitor"
	"github.com/gofiber/fiber/v2/middleware/recover"

	//"github.com/yurikrupnik/nx-go-playground/my-lib"
	"log"
	go_fiber_helpers "rust-playground/libs/go/fiber-helpers"
	go_models "rust-playground/libs/go/models"
	go_mongodb "rust-playground/libs/go/mongodb"
	go_myutils "rust-playground/libs/go/myutils"

	"go.mongodb.org/mongo-driver/bson/primitive"
)

type Project struct {
	ID   primitive.ObjectID `json:"id" bson:"_id,omitempty"`
	Name string             `json:"name" bson:"name,omitempty" validate:"required,min=3,max=36"`
}

// @securityDefinitions.basic  BasicAuth
func main() {
	// Connect to the database
	if err := go_mongodb.Connect(); err != nil {
		log.Println("failed to connect")
		log.Fatal(err)
	}

	app := fiber.New(fiber.Config{
		ErrorHandler: go_fiber_helpers.DefaultErrorHandler,
	})

	app.Use(recover.New())
	// Default middleware config
	app.Use(logger.New())
	//app.Use(csrf.New()) // todo check it - forbidden post events
	app.Use(cors.New())
	apiGroup := app.Group("api")
	apiGroup1 := app.Group("v1")
	apiGroup1.Get("/aris", func(ctx *fiber.Ctx) error {
		return ctx.SendString("Arosss")
	})
	apiGroup1.Get("/friends", func(ctx *fiber.Ctx) error {
		return ctx.SendString("dass")
	})
	apiGroup1.Get("/sap", func(ctx *fiber.Ctx) error {
		return ctx.SendString("Yes")
	})
	//go_models_user.CreateFakeGroup[users.User](apiGroup, "users")
	go_models.CreateFakeGroup[Project](apiGroup, "projects")
	go_models.CreateFakeGroup[go_models.User](apiGroup, "users")

	app.Get("/dashboard", monitor.New())
	port := go_myutils.Getenv("PORT", "8081")
	log.Println("port", port)
	host := go_myutils.Getenv("HOST", "0.0.0.0")
	result := fmt.Sprintf("%s:%s", host, port)
	log.Panic(app.Listen(result))
}
