import "dotenv/config";
import express from "express"

const app  = express();

    app.get("/hi",(req,res)=>{
        res.json({
            message : "Hi There"
        })
    })
app.listen(process.env.PORT,()=>{
    console.log(`Server is Running on ${process.env.PORT}`);
})