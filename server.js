const express = require("express");
const app = express();
// port for the server
const port = 4000;
// Run server for testing in the dist file
app.use(express.static("dist"));

app.listen(port, () => console.log(`Test started, OctaneSite running at http://localhost:${port}`));
