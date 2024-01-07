#[derive(Debug, Clone)]
pub struct QueryParamsBuilder {
    skip_count: Option<(String, String)>,
    omit_total_items: Option<(String, String)>, // bool
    order_by: Option<Vec<(String, String)>>,
    max_items: Option<(String, String)>,
    where_filter: Option<(String, String)>,
    include: Option<Vec<(String, String)>>,
    fields: Option<Vec<(String, String)>>,
}

type QueryParams = Vec<(String, String)>;

impl QueryParamsBuilder {
    pub fn new() -> Self {
        QueryParamsBuilder {
            skip_count: None,
            omit_total_items: None,
            order_by: None,
            max_items: None,
            where_filter: None,
            include: None,
            fields: None,
        }
    }

    pub fn set_skip_count(mut self, value: Option<&u32>) -> Self {
        if let Some(skip) = value {
            self.skip_count = Some(("skipCount".to_string(), skip.to_string()));
        }

        self
    }

    pub fn set_omit_total_items(mut self, value: Option<&bool>) -> Self {
        if let Some(omit) = value {
            self.omit_total_items = Some(("omitTotalItems".to_string(), omit.to_string()));
        }

        self
    }

    pub fn push_order_by(mut self, value: Option<&str>) -> Self {
        if let Some(order) = value {
            match self.order_by {
                Some(ref mut order_by) => order_by.push(("orderBy".to_string(), order.to_string())),
                None => self.fields = Some(vec![("orderBy".to_string(), order.to_string())]),
            }
        }

        self
    }

    pub fn set_max_items(mut self, value: Option<&u32>) -> Self {
        if let Some(max) = value {
            self.max_items = Some(("maxItems".to_string(), max.to_string()));
        }

        self
    }

    pub fn set_where_filter(mut self, value: Option<&String>) -> Self {
        if let Some(filter) = value {
            self.where_filter = Some(("where".to_string(), filter.to_string()));
        }

        self
    }

    pub fn push_field(mut self, value: Option<&str>) -> QueryParamsBuilder {
        if let Some(field) = value {
            match self.fields {
                Some(ref mut fields) => fields.push(("field".to_string(), field.to_string())),
                None => self.fields = Some(vec![("field".to_string(), field.to_string())]),
            }
        }

        self
    }

    pub fn push_include(mut self, value: Option<&str>) -> QueryParamsBuilder {
        if let Some(include_value) = value {
            match self.include {
                Some(ref mut include) => include.push(("include".to_string(), include_value.to_string())),
                None => self.include = Some(vec![("include".to_string(), include_value.to_string())]),
            }
        }

        self
    }

    pub fn get_skip_count(&mut self) -> &Option<(String, String)> { &self.skip_count }

    pub fn get_omit_total_items(&mut self) -> &Option<(String, String)> { &self.omit_total_items }

    pub fn get_order_by(&mut self) -> &Option<Vec<(String, String)>> { &self.order_by }

    pub fn get_max_items(&mut self) -> &Option<(String, String)> { &self.max_items }

    pub fn get_where_filter(&mut self) -> &Option<(String, String)> { &self.where_filter }

    pub fn get_fields(&mut self) -> &Option<Vec<(String,String)>> { &self.fields }

    pub fn get_include(&mut self) -> &Option<Vec<(String, String)>> { &self.include }



    pub fn build(&mut self) -> Vec<(String, String)> {
        let mut vector = Vec::new();

        if let Some(skip_count) = (&self.skip_count) {
            vector.push(skip_count.clone());
        };

        if let Some(max_items) = &self.max_items {
            vector.push(max_items.clone());
        };

        if let Some(fields) = &self.fields {
            vector.extend(fields.clone());
        };

        vector
    }
}