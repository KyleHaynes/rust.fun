# Re rust format: https://docs.rs/chrono/latest/chrono/format/strftime/index.html

#' @export
formatd <- function(date, format = ""){
    if(class(date) == "Date"){
        date <- as.integer(date)
        date[!is.na(date)] <- r_format_date(date[!is.na(date)], format)
    } else if(class(date) == "character"){
        date[!is.na(date)] <- r_format_cdate(date[!is.na(date)], format)
    } else {
        stop("`date` must be of class `Date` or `character`.")
    }
    return(date)
}