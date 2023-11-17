# Asynchronous Asset Upload System in Rust

The `fn upload_asset` function operates asynchronously, showcasing the ability to handling  multipart/form-data requests in a web application by allowing you to parse the request body into a type-safe struct.

 - File Handling: Precise extraction of data such as the file name and author of the image.
Ordered construction of the file path in the `./tmp/author/asset.jpg` directory.

 - Local File Upload: Effective persistence of the image content in the local file system.

 - Cloud Upload: Invocation of a specialized asynchronous function to upload the asset to Cloudinary Storage.

 - Temporary File Cleanup:Proactive strategy for removing the temporary directory and its content after upload operations.

 - Result Handling: Rigorous evaluation of the Cloudinary upload result.
Return of corresponding status codes indicating success or potential setbacks.


This function `fn upload_asset` serves as the backbone of our project, highlighting our ability to professionally manage the asynchronous upload of assets, ensuring an efficient and secure workflow both locally and in the cloud.

