/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2025 Kovács Dávid <kapcsolat@kovacsdavid.dev>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::common::dto::{EmptyType, SimpleMessageResponse, SuccessResponseBuilder, UuidParam};
use crate::common::extractors::UserInput;
use crate::common::handler::{ErrorMapper, ErrorMapperInterface, HandlerResult};
use crate::common::query_parser::{CommonRawQuery, ResourceQuery};
use crate::common::service::Service;
use crate::manager::auth::middleware::AuthenticatedUser;
use crate::tenant::customers::CustomersModule;
use crate::tenant::customers::dto::print::CustomerResolvedPrint;
use crate::tenant::customers::dto::user_input::{CustomerUserInput, CustomerUserInputHelper};
use crate::tenant::customers::service::CustomerService;
use crate::tenant::customers::types::customer::{CustomerFilterBy, CustomerOrderBy};
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use std::str::FromStr;
use std::sync::Arc;

pub async fn get_resolved<M: CustomersModule>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    let error_mapper = ErrorMapper::new(customers_module);
    let result = error_mapper
        .or_handler_error(service.get_resolved(payload.uuid).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(result)
                .build(),
        )
        .await?
        .into_response())
}

pub async fn get<M: CustomersModule>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    let error_mapper = ErrorMapper::new(customers_module);
    let result = error_mapper
        .or_handler_error(service.get(payload.uuid).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(result)
                .build(),
        )
        .await?
        .into_response())
}

pub async fn create<M: CustomersModule>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<CustomerUserInput, CustomerUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    let error_mapper = ErrorMapper::new(customers_module);
    let result = error_mapper
        .or_handler_error(service.insert(&user_input).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::CREATED)
                .data(result)
                .build(),
        )
        .await?
        .into_response())
}

pub async fn update<M: CustomersModule>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    UserInput(user_input, _): UserInput<CustomerUserInput, CustomerUserInputHelper>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    let error_mapper = ErrorMapper::new(customers_module);
    let result = error_mapper
        .or_handler_error(service.update(&user_input).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(result)
                .build(),
        )
        .await?
        .into_response())
}

pub async fn delete<M: CustomersModule>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    let error_mapper = ErrorMapper::new(customers_module);
    error_mapper
        .or_handler_error(service.delete(payload.uuid).await)
        .await?;
    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::<EmptyType, _>::new()
                .status_code(StatusCode::OK)
                .data(SimpleMessageResponse::new(
                    "A vevő törlése sikeresen megtörtént",
                ))
                .build(),
        )
        .await?
        .into_response())
}

pub async fn list<M: CustomersModule>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    Query(payload): Query<CommonRawQuery>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    let error_mapper = ErrorMapper::new(customers_module);
    let resource_query = error_mapper
        .or_handler_error(ResourceQuery::<CustomerOrderBy, CustomerFilterBy>::from_str(payload.q()))
        .await?;
    let (meta, data) = error_mapper
        .or_handler_error(service.get_paged(&resource_query).await)
        .await?;

    Ok(error_mapper
        .or_handler_error(
            SuccessResponseBuilder::new()
                .status_code(StatusCode::OK)
                .meta(meta)
                .data(data)
                .build(),
        )
        .await?
        .into_response())
}

pub async fn print<M: CustomersModule>(
    AuthenticatedUser(claims): AuthenticatedUser,
    State(customers_module): State<Arc<M>>,
    Query(payload): Query<UuidParam>,
) -> HandlerResult {
    let service = Service::new(Some(&claims), customers_module.clone());
    let error_mapper = ErrorMapper::new(customers_module);
    let customer_resolved_print = CustomerResolvedPrint::from_customer_revolved(
        error_mapper
            .or_handler_error(service.get_resolved(payload.uuid).await)
            .await?,
        error_mapper.or_handler_error(claims.tz()).await?,
    );
    let pdf = error_mapper
        .or_handler_error(service.print(&[customer_resolved_print]).await)
        .await?;
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        r#"inline; filename="invoice.pdf""#.parse().unwrap(),
    );
    Ok((StatusCode::OK, headers, pdf).into_response())
}
