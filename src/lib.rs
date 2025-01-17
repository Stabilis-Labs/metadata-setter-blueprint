use scrypto::prelude::*;

#[blueprint]
mod metadata_setter {
    enable_method_auth! {
        methods {
            insert_admin_badge => PUBLIC;
            take_admin_badge => restrict_to: [OWNER];
            update_resource_metadata_field => restrict_to: [OWNER];
            update_account_metadata_field => restrict_to: [OWNER];
        }
    }

    struct MetadataSetter {
        admin_badge: Vault,
    }

    impl MetadataSetter {
        pub fn instantiate_metadata_setter(
            admin_badge_address: ResourceAddress,
            owner_role: OwnerRole,
        ) -> Global<MetadataSetter> {
            Self {
                admin_badge: Vault::new(admin_badge_address),
            }
            .instantiate()
            .prepare_to_globalize(owner_role)
            .globalize()
        }

        pub fn insert_admin_badge(&mut self, badge: Bucket) {
            self.admin_badge.put(badge);
        }

        pub fn take_admin_badge(&mut self, amount: Decimal) -> Bucket {
            self.admin_badge.take(amount)
        }

        pub fn update_resource_metadata_field(
            &mut self,
            resource: ResourceAddress,
            field_name: String,
            field_value: MetadataValue,
        ) {
            let rm = ResourceManager::from(resource);

            self.admin_badge
                .as_fungible()
                .authorize_with_amount(dec!(1), || match field_value {
                    MetadataValue::String(value) => rm.set_metadata(field_name, value),
                    MetadataValue::Bool(value) => rm.set_metadata(field_name, value),
                    MetadataValue::U8(value) => rm.set_metadata(field_name, value),
                    MetadataValue::U32(value) => rm.set_metadata(field_name, value),
                    MetadataValue::U64(value) => rm.set_metadata(field_name, value),
                    MetadataValue::I32(value) => rm.set_metadata(field_name, value),
                    MetadataValue::I64(value) => rm.set_metadata(field_name, value),
                    MetadataValue::Decimal(value) => rm.set_metadata(field_name, value),
                    MetadataValue::GlobalAddress(value) => rm.set_metadata(field_name, value),
                    MetadataValue::PublicKey(value) => rm.set_metadata(field_name, value),
                    MetadataValue::NonFungibleGlobalId(value) => rm.set_metadata(field_name, value),
                    MetadataValue::NonFungibleLocalId(value) => rm.set_metadata(field_name, value),
                    MetadataValue::Instant(value) => rm.set_metadata(field_name, value),
                    MetadataValue::Url(value) => rm.set_metadata(field_name, value),
                    MetadataValue::Origin(value) => rm.set_metadata(field_name, value),
                    MetadataValue::PublicKeyHash(value) => rm.set_metadata(field_name, value),
                    MetadataValue::StringArray(value) => rm.set_metadata(field_name, value),
                    MetadataValue::BoolArray(value) => rm.set_metadata(field_name, value),
                    MetadataValue::U8Array(value) => rm.set_metadata(field_name, value),
                    MetadataValue::U32Array(value) => rm.set_metadata(field_name, value),
                    MetadataValue::U64Array(value) => rm.set_metadata(field_name, value),
                    MetadataValue::I32Array(value) => rm.set_metadata(field_name, value),
                    MetadataValue::I64Array(value) => rm.set_metadata(field_name, value),
                    MetadataValue::DecimalArray(value) => rm.set_metadata(field_name, value),
                    MetadataValue::GlobalAddressArray(value) => rm.set_metadata(field_name, value),
                    MetadataValue::PublicKeyArray(value) => rm.set_metadata(field_name, value),
                    MetadataValue::NonFungibleGlobalIdArray(value) => {
                        rm.set_metadata(field_name, value)
                    }
                    MetadataValue::NonFungibleLocalIdArray(value) => {
                        rm.set_metadata(field_name, value)
                    }
                    MetadataValue::InstantArray(value) => rm.set_metadata(field_name, value),
                    MetadataValue::UrlArray(value) => rm.set_metadata(field_name, value),
                    MetadataValue::OriginArray(value) => rm.set_metadata(field_name, value),
                    MetadataValue::PublicKeyHashArray(value) => rm.set_metadata(field_name, value),
                });
        }

        pub fn update_account_metadata_field(
            &mut self,
            component: ComponentAddress,
            field_name: String,
            field_value: MetadataValue,
        ) {
            let component = Global::<Account>::from(component);

            self.admin_badge
                .as_fungible().authorize_with_amount(dec!(1), || match field_value {
                    MetadataValue::String(value) => component.set_metadata(field_name, value),
                    MetadataValue::Bool(value) => component.set_metadata(field_name, value),
                    MetadataValue::U8(value) => component.set_metadata(field_name, value),
                    MetadataValue::U32(value) => component.set_metadata(field_name, value),
                    MetadataValue::U64(value) => component.set_metadata(field_name, value),
                    MetadataValue::I32(value) => component.set_metadata(field_name, value),
                    MetadataValue::I64(value) => component.set_metadata(field_name, value),
                    MetadataValue::Decimal(value) => component.set_metadata(field_name, value),
                    MetadataValue::GlobalAddress(value) => {
                        component.set_metadata(field_name, value)
                    }
                    MetadataValue::PublicKey(value) => component.set_metadata(field_name, value),
                    MetadataValue::NonFungibleGlobalId(value) => {
                        component.set_metadata(field_name, value)
                    }
                    MetadataValue::NonFungibleLocalId(value) => {
                        component.set_metadata(field_name, value)
                    }
                    MetadataValue::Instant(value) => component.set_metadata(field_name, value),
                    MetadataValue::Url(value) => component.set_metadata(field_name, value),
                    MetadataValue::Origin(value) => component.set_metadata(field_name, value),
                    MetadataValue::PublicKeyHash(value) => {
                        component.set_metadata(field_name, value)
                    }
                    MetadataValue::StringArray(value) => component.set_metadata(field_name, value),
                    MetadataValue::BoolArray(value) => component.set_metadata(field_name, value),
                    MetadataValue::U8Array(value) => component.set_metadata(field_name, value),
                    MetadataValue::U32Array(value) => component.set_metadata(field_name, value),
                    MetadataValue::U64Array(value) => component.set_metadata(field_name, value),
                    MetadataValue::I32Array(value) => component.set_metadata(field_name, value),
                    MetadataValue::I64Array(value) => component.set_metadata(field_name, value),
                    MetadataValue::DecimalArray(value) => component.set_metadata(field_name, value),
                    MetadataValue::GlobalAddressArray(value) => {
                        component.set_metadata(field_name, value)
                    }
                    MetadataValue::PublicKeyArray(value) => {
                        component.set_metadata(field_name, value)
                    }
                    MetadataValue::NonFungibleGlobalIdArray(value) => {
                        component.set_metadata(field_name, value)
                    }
                    MetadataValue::NonFungibleLocalIdArray(value) => {
                        component.set_metadata(field_name, value)
                    }
                    MetadataValue::InstantArray(value) => component.set_metadata(field_name, value),
                    MetadataValue::UrlArray(value) => component.set_metadata(field_name, value),
                    MetadataValue::OriginArray(value) => component.set_metadata(field_name, value),
                    MetadataValue::PublicKeyHashArray(value) => {
                        component.set_metadata(field_name, value)
                    }
                });
        }
    }
}
