package go_mongodb

import (
	"context"
	"fmt"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	go_myutils "rust-playground/libs/go/myutils"
	"time"
)

// todo remove!!!!
var mongoUrl string = "mongodb+srv://yurikrupnik:T4eXKj1RBI4VnszC@cluster0.rdmew.mongodb.net/"

// todo add back for env vars
//var mongoUrl = go_myutils.Getenv("MONGO_URI", "mongodb://localhost/mussia12")
var dbName = go_myutils.Getenv("DB_NAME", "mussia12")

//MongoInstance contains the Mongo client and database objects
type MongoInstance struct {
	Client *mongo.Client
	Db     *mongo.Database
}

var Mg MongoInstance

//func (m *MongoInstance) Get(it int) {
//	cursor, err := m.Db.Collection("users").Find(ctx, query)
//	if err != nil {
//		return c.Status(http.StatusInternalServerError).SendString(err.Error())
//	}
//	//defer cursor.Close(ctx)
//	var users []User = make([]User, 0)
//
//	// iterate the cursor and decode each item into an Employee
//	//if err := cursor.All(c.Context(), &users); err != nil {
//	//  return c.Status(http.StatusInternalServerError).SendString(err.Error())
//	//}
//
//	for cursor.Next(ctx) {
//		var user User
//		err := cursor.Decode(&user)
//		if err != nil {
//			fmt.Println(err)
//			//return err
//			//log.Fatal("fatal")
//			//log.Panic("panic")
//		}
//		users = append(users, user)
//	}
//	// return employees list in JSON format
//	return c.Status(http.StatusOK).JSON(users)
//}

func NewDB() MongoInstance {
	client, err := mongo.NewClient(options.Client().ApplyURI(mongoUrl))
	if err != nil {
		fmt.Println("error connecting to mongo:", err)
	}
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()
	err = client.Connect(ctx)
	db := client.Database(dbName)
	return MongoInstance{
		Client: client,
		Db:     db,
	}
}

// Connect configures the MongoDB client and initializes the database connection.
// Source: https://www.mongodb.com/blog/post/quick-start-golang--mongodb--starting-and-setup
func Connect() error {
	client, err := mongo.NewClient(options.Client().ApplyURI(mongoUrl))

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	err = client.Connect(ctx)
	db := client.Database(dbName)

	if err != nil {
		return err
	}

	Mg = MongoInstance{
		Client: client,
		Db:     db,
	}

	return nil
}
