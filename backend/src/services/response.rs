use iron::IronResult;
use iron::Response as IronResponse;
use iron::status::Status;
// use json::Error;
use std::convert::Into;
use services::error::ErrorResponse;

pub fn new<S>(status: Status, res: S) -> IronResult<IronResponse>
    where S: Into<String>
{
    let r: String = res.into();
    Ok(IronResponse::with((status, r)))

}

pub fn no_content() -> IronResult<IronResponse> {
    Ok(IronResponse::with((Status::NoContent, "")))

}

pub fn ok<S>(res: S) -> IronResult<IronResponse>
    where S: Into<String>
{
    let r: String = res.into();
    Ok(IronResponse::with((Status::Ok, r)))

}

pub fn bad_request<S>(err: S) -> IronResult<IronResponse>
    where S: Into<String>
{
    let e = ErrorResponse::bad_request(err);
    Ok(IronResponse::with((e.status(), e.return_as_json())))

}

pub fn internal_error<S>(err: S) -> IronResult<IronResponse>
    where S: Into<String>
{
    let e = ErrorResponse::internal_error(err);
    Ok(IronResponse::with((e.status(), e.return_as_json())))

}

pub fn not_found<S>(err: S) -> IronResult<IronResponse>
    where S: Into<String>
{
    let e = ErrorResponse::not_found(err);
    Ok(IronResponse::with((e.status(), e.return_as_json())))

}

pub fn unauthorized<S>(err: S) -> IronResult<IronResponse>
    where S: Into<String>
{
    let e = ErrorResponse::unauthorized(err);
    Ok(IronResponse::with((e.status(), e.return_as_json())))

}
