package go_models

import (
	"context"
	"fmt"
	"github.com/gofiber/fiber/v2"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"net/http"
	go_mongodb "rust-playground/libs/go/mongodb"
	"time"
)

type ApiMethods struct {
	create  func()
	getById func()
	getList func()
	remove  func()
	update  func()
}

type User struct {
	ID    string `json:"id,omitempty" bson:"_id,omitempty"`
	Email string `json:"email,omitempty" bson:"email,omitempty"`
	Name  string `json:"name"`
	Role  string `json:"role"`
	//Role float64 `json:"salary"`
	//Age    float64 `json:"age"`
}

func CreateFakeGroup[T interface{}](api fiber.Router, name string) {
	router := api.Group(name)
	router.Post("", CreateHandlerCreate[T]())
	router.Get("", CreateHandlerList[T]())
	router.Get("/:id", CreateHandleGetById[T]())
	router.Delete("/:id", CreateHandleDeleteById[T]())
	router.Put("/:id", CreateHandleUpdate[T]())
}

func createController[T interface{}](c *fiber.Ctx) error {
	var user T
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	//validate the request body
	if err := c.BodyParser(&user); err != nil {
		return c.Status(http.StatusBadRequest).JSON(err.Error())
	}
	fmt.Println(">>>>>>user", user)
	result, err := go_mongodb.Mg.Db.Collection("users").InsertOne(ctx, user)
	if err != nil {
		//return c.Status(http.StatusInternalServerError).JSON(UserResponse{Status: http.StatusInternalServerError, Message: "error", Data: &fiber.Map{"data": err.Error()}})
	}
	fmt.Println("result", result)
	fmt.Println(">>>>>>user", user)
	return c.Status(http.StatusCreated).JSON(result)
}

func CreateHandlerCreate[T interface{}]() fiber.Handler {
	return createController[T]
}

func CreateHandlerList[T interface{}]() fiber.Handler {
	return func(c *fiber.Ctx) error {

		//var usr T
		ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
		defer cancel()
		//todo handle query
		//data := c.Request().URI().QueryString()
		query := bson.M{} // todo handle query
		//query := bson.M{} // todo handle query
		//fmt.Println(c.Query("email"))
		//fmt.Println(c.Query("*"))
		cursor, err := go_mongodb.Mg.Db.Collection("users").Find(ctx, query)
		if err != nil {
			return c.Status(http.StatusInternalServerError).SendString(err.Error())
		}
		//defer cursor.Close(ctx)
		var response []T = make([]T, 0)

		// iterate the cursor and decode each item into an Employee
		//if err := cursor.All(c.Context(), &users); err != nil {
		//  return c.Status(http.StatusInternalServerError).SendString(err.Error())
		//}

		for cursor.Next(ctx) {
			var item T
			err := cursor.Decode(&item)
			if err != nil {
				fmt.Println(err)
			}
			response = append(response, item)
		}
		// return employees list in JSON format
		return c.Status(http.StatusOK).JSON(response)
	}
}

func CreateHandleGetById[T interface{}]() fiber.Handler {
	return func(c *fiber.Ctx) error {
		ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
		id := c.Params("id")
		var response T
		defer cancel()

		objId, _ := primitive.ObjectIDFromHex(id)

		err := go_mongodb.Mg.Db.Collection("users").FindOne(ctx, bson.M{"_id": objId}).Decode(&response)
		if err != nil {
			return c.Status(http.StatusInternalServerError).JSON(err.Error())
		}

		//return c.Status(http.StatusOK).JSON(UserResponse{Status: http.StatusOK, Message: "success", Data: &fiber.Map{"data": user}})
		return c.Status(fiber.StatusOK).JSON(response)
	}
}

func CreateHandleDeleteById[T interface{}]() fiber.Handler {
	return func(c *fiber.Ctx) error {
		ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
		userId := c.Params("id")
		fmt.Println("user id>>", userId)
		defer cancel()

		objId, _ := primitive.ObjectIDFromHex(userId)

		//result, err := userCollection.DeleteOne(ctx, bson.M{"id": objId})
		result, err := go_mongodb.Mg.Db.Collection("users").DeleteOne(ctx, bson.M{"_id": objId})
		if err != nil {
			return c.Status(http.StatusInternalServerError).JSON(err.Error())
		}

		if result.DeletedCount < 1 {
			return c.Status(http.StatusNotFound).JSON(
				&fiber.Map{"data": "User with specified ID not found!"},
			)
		}

		return c.Status(http.StatusOK).JSON(
			&fiber.Map{"data": "User successfully deleted!"},
		)
	}
}

func CreateHandleUpdate[T interface{}]() fiber.Handler {
	return func(c *fiber.Ctx) error {
		ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
		id := c.Params("id")
		var user T
		defer cancel()

		objId, _ := primitive.ObjectIDFromHex(id)
		fmt.Println(objId)
		//validate the request body
		if err := c.BodyParser(&user); err != nil {
			return c.Status(http.StatusBadRequest).JSON(err.Error())
		}

		//use the validator library to validate required fields
		//if validationErr := validate.Struct(&user); validationErr != nil {
		//  return c.Status(http.StatusBadRequest).JSON(UserResponse{Status: http.StatusBadRequest, Message: "error", Data: &fiber.Map{"data": validationErr.Error()}})
		//}

		// map
		//update := bson.M{"name": user.Name, "email": user.Email, "role": user.Role}

		//fmt.Println(update)
		result, err := go_mongodb.Mg.Db.Collection("users").UpdateOne(ctx, bson.M{"_id": objId}, bson.M{"$set": user})

		if err != nil {
			fmt.Println(err)
			return c.Status(http.StatusInternalServerError).JSON(err.Error())
		}
		//get updated user details
		var updatedUser T
		//fmt.Println("result", result.)
		if result.MatchedCount == 1 {
			fmt.Println("dmmm")
			err := go_mongodb.Mg.Db.Collection("users").FindOne(ctx, bson.M{"_id": objId}).Decode(&updatedUser)

			if err != nil {
				fmt.Println("dmmm11")
				return c.Status(http.StatusInternalServerError).JSON(err.Error())
			}
		}

		return c.Status(http.StatusOK).JSON(updatedUser)
		//result, err := userCollection.DeleteOne(ctx, bson.M{"id": objId})
		//result, err := go_mongodb.Mg.Db.Collection("users").DeleteOne(ctx, bson.M{"id": objId})
		//if err != nil {
		//  return c.Status(http.StatusInternalServerError).JSON(UserResponse{Status: http.StatusInternalServerError, Message: "error", Data: &fiber.Map{"data": err.Error()}})
		//}
		//
		//if result.DeletedCount < 1 {
		//  return c.Status(http.StatusNotFound).JSON(
		//    UserResponse{Status: http.StatusNotFound, Message: "error", Data: &fiber.Map{"data": "User with specified ID not found!"}},
		//  )
		//}
		//
		//return c.Status(http.StatusOK).JSON(
		//  UserResponse{Status: http.StatusOK, Message: "success", Data: &fiber.Map{"data": "User successfully deleted!"}},
		//)
	}
}
