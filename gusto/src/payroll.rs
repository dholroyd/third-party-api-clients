use anyhow::Result;

use crate::Client;

pub struct Payroll {
    client: Client,
}

impl Payroll {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Payroll { client }
    }

    /**
     * Get pay periods for a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/pay_periods` endpoint.
     *
     * Pay periods are the foundation of payroll. Compensation, time & attendance, taxes, and expense reports all rely on when they happened. To begin submitting information for a given payroll, we need to agree on the time period.
     *
     *
     * By default, this endpoint returns every current and past pay period for a company. Since companies can process payroll as often as every week, there can be up to 53 pay periods a year. If a company has been running payroll with Gusto for five years, this endpoint could return up to 265 pay periods. Use the `start_date` and `end_date` parameters to reduce the scope of the response.
     *
     * **Parameters:**
     *
     * * `start_date: &str`
     * * `end_date: &str`
     */
    pub async fn get_company_pay_periods(
        &self,
        company_id_or_uuid: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<crate::types::PayPeriod>> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !end_date.is_empty() {
            query_args.push(format!("end_date={}", end_date));
        }
        if !start_date.is_empty() {
            query_args.push(format!("start_date={}", start_date));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/v1/companies/{}/pay_periods?{}",
            crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),
            query
        );

        self.client.get(&url).await
    }

    /**
     * Get pay periods for a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/pay_periods` endpoint.
     *
     * As opposed to `get_company_pay_periods`, this function returns all the pages of the request at once.
     *
     * Pay periods are the foundation of payroll. Compensation, time & attendance, taxes, and expense reports all rely on when they happened. To begin submitting information for a given payroll, we need to agree on the time period.
     *
     *
     * By default, this endpoint returns every current and past pay period for a company. Since companies can process payroll as often as every week, there can be up to 53 pay periods a year. If a company has been running payroll with Gusto for five years, this endpoint could return up to 265 pay periods. Use the `start_date` and `end_date` parameters to reduce the scope of the response.
     */
    pub async fn get_all_company_pay_periods(
        &self,
        company_id_or_uuid: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<crate::types::PayPeriod>> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !end_date.is_empty() {
            query_args.push(format!("end_date={}", end_date));
        }
        if !start_date.is_empty() {
            query_args.push(format!("start_date={}", start_date));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/v1/companies/{}/pay_periods?{}",
            crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),
            query
        );

        self.client.get_all_pages(&url).await
    }

    /**
     * Get all payrolls for a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/payrolls` endpoint.
     *
     * Returns all payrolls, current and past for a company.
     *
     * Notes:
     * * Hour and dollar amounts are returned as string representations of numeric decimals.
     * * Hours are represented to the thousands place; dollar amounts are represented to the cent.
     * * Every eligible compensation is returned for each employee. If no data has yet be inserted for a given field, it defaults to “0.00” (for fixed amounts) or “0.000” (for hours ).
     *
     * **Parameters:**
     *
     * * `processed: bool` -- Whether to return processed or unprocessed payrolls.
     * * `include_off_cycle: bool` -- Whether to include off cycle payrolls in the response.
     * * `include: &[String]` -- Include the requested attribute in the employee_compensations attribute in the response.
     * * `start_date: &str` -- Return payrolls whose pay period is after the start date.
     * * `end_date: &str` -- Return payrolls whose pay period is before the end date.
     */
    pub async fn get_company_payrolls(
        &self,
        company_id_or_uuid: &str,
        processed: bool,
        include_off_cycle: bool,
        include: &[String],
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<crate::types::PayrollData>> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !end_date.is_empty() {
            query_args.push(format!("end_date={}", end_date));
        }
        if !include.is_empty() {
            query_args.push(format!("include={}", include.join(" ")));
        }
        if include_off_cycle {
            query_args.push(format!("include_off_cycle={}", include_off_cycle));
        }
        if processed {
            query_args.push(format!("processed={}", processed));
        }
        if !start_date.is_empty() {
            query_args.push(format!("start_date={}", start_date));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/v1/companies/{}/payrolls?{}",
            crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),
            query
        );

        self.client.get(&url).await
    }

    /**
     * Get all payrolls for a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/payrolls` endpoint.
     *
     * As opposed to `get_company_payrolls`, this function returns all the pages of the request at once.
     *
     * Returns all payrolls, current and past for a company.
     *
     * Notes:
     * * Hour and dollar amounts are returned as string representations of numeric decimals.
     * * Hours are represented to the thousands place; dollar amounts are represented to the cent.
     * * Every eligible compensation is returned for each employee. If no data has yet be inserted for a given field, it defaults to “0.00” (for fixed amounts) or “0.000” (for hours ).
     */
    pub async fn get_all_company_payrolls(
        &self,
        company_id_or_uuid: &str,
        processed: bool,
        include_off_cycle: bool,
        include: &[String],
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<crate::types::PayrollData>> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !end_date.is_empty() {
            query_args.push(format!("end_date={}", end_date));
        }
        if !include.is_empty() {
            query_args.push(format!("include={}", include.join(" ")));
        }
        if include_off_cycle {
            query_args.push(format!("include_off_cycle={}", include_off_cycle));
        }
        if processed {
            query_args.push(format!("processed={}", processed));
        }
        if !start_date.is_empty() {
            query_args.push(format!("start_date={}", start_date));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/v1/companies/{}/payrolls?{}",
            crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),
            query
        );

        self.client.get_all_pages(&url).await
    }

    /**
     * Create an Off-Cycle Payroll (Beta).
     *
     * This function performs a `POST` to the `/v1/companies/{company_id_or_uuid}/payrolls` endpoint.
     *
     * **This endpoint is in beta. Please contact developer-gws@gusto.com if you’d like to have more information and use it for production. Note, this may require you to enter a different agreement with Gusto.**
     *
     * Creates a new, unprocessed, off-cycle payroll.
     */
    pub async fn post_company_payrolls(
        &self,
        company_id_or_uuid: &str,
        body: &crate::types::PostCompanyPayrollsRequest,
    ) -> Result<crate::types::PayrollData> {
        let url = format!(
            "/v1/companies/{}/payrolls",
            crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Get a single payroll.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/payrolls/{payroll_id_or_uuid}` endpoint.
     *
     * Returns a payroll.
     *
     * Notes:
     * * Hour and dollar amounts are returned as string representations of numeric decimals.
     * * Hours are represented to the thousands place; dollar amounts are represented to the cent.
     * * Every eligible compensation is returned for each employee. If no data has yet be inserted for a given field, it defaults to “0.00” (for fixed amounts) or “0.000” (for hours ).
     *
     * **Parameters:**
     *
     * * `include: crate::types::GetCompanyPayrollsInclude` -- Include the requested attribute in the employee_compensations attribute in the response.
     * * `show_calculation: &str` -- with `include`, shows the tax, and/or benefit, and/or deduction details for a calculated, unprocessed payroll.
     */
    pub async fn get_company_payrolls_payroll_id(
        &self,
        company_id_or_uuid: &str,
        payroll_id_or_uuid: &str,
        include: crate::types::GetCompanyPayrollsInclude,
        show_calculation: &str,
    ) -> Result<crate::types::PayrollData> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("include={}", include));
        if !show_calculation.is_empty() {
            query_args.push(format!("show_calculation={}", show_calculation));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/v1/companies/{}/payrolls/{}?{}",
            crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),
            crate::progenitor_support::encode_path(&payroll_id_or_uuid.to_string()),
            query
        );

        self.client.get(&url).await
    }

    /**
     * Update a payroll by ID.
     *
     * This function performs a `PUT` to the `/v1/companies/{company_id_or_uuid}/payrolls/{payroll_id_or_uuid}` endpoint.
     *
     * This endpoint allows you to update information for one or more employees for a specific **unprocessed** payroll.
     */
    pub async fn put_company_payrolls(
        &self,
        company_id_or_uuid: &str,
        payroll_id_or_uuid: &str,
        body: &crate::types::PutCompanyPayrollsRequest,
    ) -> Result<crate::types::PayrollData> {
        let url = format!(
            "/v1/companies/{}/payrolls/{}",
            crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),
            crate::progenitor_support::encode_path(&payroll_id_or_uuid.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Update a payroll.
     *
     * This function performs a `PUT` to the `/v1/companies/{company_id_or_uuid}/payrolls/{pay_period_start_date}/{pay_period_end_date}` endpoint.
     *
     * This endpoint allows you to update information for one or more employees for a specific **unprocessed** payroll.
     *
     * The payrolls are identified by their pay periods’ start_date and end_date. Both are required and must correspond with an existing, unprocessed payroll. *If the dates do not match, the entire request will be rejected.* This was an explicit design decision to remove any assumptions around the timespan for data sent.
     */
    pub async fn put_company_payrolls_pay_period_start_date_pay_period_end_date(
        &self,
        company_id_or_uuid: &str,
        pay_period_start_date: &str,
        pay_period_end_date: &str,
        body: &crate::types::PutCompanyPayrollsRequest,
    ) -> Result<crate::types::PayrollData> {
        let url = format!(
            "/v1/companies/{}/payrolls/{}/{}",
            crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),
            crate::progenitor_support::encode_path(&pay_period_start_date.to_string()),
            crate::progenitor_support::encode_path(&pay_period_end_date.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Calculate a Payroll (Beta).
     *
     * This function performs a `PUT` to the `/v1/companies/{company_id}/payrolls/{payroll_id}/calculate` endpoint.
     *
     * **This endpoint is in beta. Please contact developer-gws@gusto.com if you’d like to have more information and use it for production. Note, this may require you to enter a different agreement with Gusto.**
     *
     * Performs calculations for taxes, benefits, and deductions for an unprocessed payroll. The calculated payroll details provide a preview of the actual values that will be used when the payroll is run.
     *
     * This endpoint is asynchronous and responds with only a 202 HTTP status. To view the details of the calculated payroll, use the GET /v1/companies/{company_id}/payrolls/{payroll_id} endpoint with the *show_calculation* and *includes* params
     */
    pub async fn put_company_payrolls_payroll_id_calculate(
        &self,
        company_id: &str,
        payroll_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/v1/companies/{}/payrolls/{}/calculate",
            crate::progenitor_support::encode_path(&company_id.to_string()),
            crate::progenitor_support::encode_path(&payroll_id.to_string()),
        );

        self.client.put(&url, None).await
    }

    /**
     * Submit Payroll (Beta).
     *
     * This function performs a `PUT` to the `/v1/companies/{company_id}/payrolls/{payroll_Id}/submit` endpoint.
     *
     * **This endpoint is in beta. Please contact developer-gws@gusto.com if you’d like to have more information and use it for production. Note, this may require you to enter a different agreement with Gusto.**
     *
     * Submits an unprocessed payroll to be calculated and run. Upon success, transitions the payroll to the `processed` state.
     */
    pub async fn put_company_payrolls_payroll_id_submit(
        &self,
        company_id: &str,
        payroll_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/v1/companies/{}/payrolls/{}/submit",
            crate::progenitor_support::encode_path(&company_id.to_string()),
            crate::progenitor_support::encode_path(&payroll_id.to_string()),
        );

        self.client.put(&url, None).await
    }

    /**
     * Cancel a Payroll (Beta).
     *
     * This function performs a `PUT` to the `/v1/companies/{company_id}/payrolls/{payroll_id}/cancel` endpoint.
     *
     * **This endpoint is in beta. Please contact developer-gws@gusto.com if you’d like to have more information and use it for production. Note, this may require you to enter a different agreement with Gusto.**
     *
     * Transitions a `processed` payroll back to the `unprocessed` state. A payroll cannot be canceled once it has entered the `funded` state.
     *
     */
    pub async fn put_api_company_payrolls_payroll_id_cancel(
        &self,
        company_id: &str,
        payroll_id: &str,
    ) -> Result<crate::types::PayrollData> {
        let url = format!(
            "/v1/companies/{}/payrolls/{}/cancel",
            crate::progenitor_support::encode_path(&company_id.to_string()),
            crate::progenitor_support::encode_path(&payroll_id.to_string()),
        );

        self.client.put(&url, None).await
    }

    /**
     * Get approved Payroll Reversals.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/payroll_reversals` endpoint.
     *
     * Returns all approved Payroll Reversals for a Company.
     */
    pub async fn get_company_or_uuid_payroll_reversals(
        &self,
        company_id_or_uuid: &str,
    ) -> Result<crate::types::GetCompanyUuidPayrollReversalsResponse> {
        let url = format!(
            "/v1/companies/{}/payroll_reversals",
            crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),
        );

        self.client.get(&url).await
    }
}
