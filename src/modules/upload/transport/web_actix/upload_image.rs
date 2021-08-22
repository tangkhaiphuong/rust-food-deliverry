use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};

use crate::component::AppContext;
use crate::modules::restaurant::transport::web_actix::{error, simple_json};
use crate::modules::upload::business::new_update_image_business;

pub fn upload_image(
    app_context: web::Data<Box<dyn AppContext>>,
    mut _payload: Multipart,
    _request: HttpRequest,
) -> HttpResponse {
    // fileHeader, err := c.FormFile("file")
    //
    // if err != nil {
    //     c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
    //     return
    // }
    //
    // folder := c.DefaultPostForm("folder", "img")
    //
    // file, err := fileHeader.Open()
    //
    // if err != nil {
    //     c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
    //     return
    // }
    //
    // defer file.Close() // we can close here
    //
    // dataBytes := make([]byte, fileHeader.Size)
    // if _, err := file.Read(dataBytes); err != nil {
    //     c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
    //     return
    // }
    //

    // //imgStore := uploadstorage.NewSQLStore(db)
    let upload_provider = app_context.upload_provider();
    let business = new_update_image_business(upload_provider.clone());

    let data = [0u8; 24];
    let result = business.save_file_uploaded(&data, "".to_string());

    return match result {
        Err(err) => error(err),
        Ok(image) => {
            return simple_json(image);
        }
    };
}
