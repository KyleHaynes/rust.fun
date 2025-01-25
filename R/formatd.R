#' @export
formatd <- function(date, format = ""){
    if(class(date) != "character"){
        date <- as.character(date)
    }

    date[!is.na(date)] <- r_format_cdate(date[!is.na(date)], format)


    return(date)
}