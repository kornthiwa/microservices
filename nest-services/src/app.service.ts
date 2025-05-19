import { Injectable } from '@nestjs/common';
import { InternalServerErrorException } from '@nestjs/common';
import axios from 'axios';

@Injectable()
export class AppService {
  async getProductTemplate(productId: number) {
    try {
      const { isValid, sessionId } = await authenticateOdoo();

      if (isValid && sessionId) {
        const odooURL = `${ODOO_BASE_URL}/web/dataset/call_kw/product.template/read`;
        const params = {
          jsonrpc: '2.0',
          method: 'call',
          params: {
            args: [
              [productId],
              [
                'product_variant_count',
                'is_product_variant',
                'attribute_line_ids',
                'meditem_count',
                'purchased_product_qty',
                'uom_name',
                'sh_secondary_uom_onhand',
                'sh_uom_name',
                'sh_secondary_uom_forecasted',
                'qty_available',
                'virtual_available',
                'immediately_usable_qty',
                'potential_qty',
                'reordering_min_qty',
                'reordering_max_qty',
                'nbr_reordering_rules',
                'bom_count',
                'used_in_bom_count',
                'mrp_product_qty',
                'sales_count',
                'id',
                'image_1920',
                '__last_update',
                'qr_code',
                'name',
                'sale_ok',
                'is_product_returned',
                'is_agency_st',
                'is_uc_st',
                'can_be_consign',
                'purchase_ok',
                'isParts',
                'active',
                'type',
                'categ_id',
                'default_code',
                'barcode_type',
                'barcode_open_id',
                'barcode_department_id',
                'barcode',
                'sh_qr_code',
                'sh_qr_code_img',
                'medcode_medgrp_id',
                'list_price',
                'pricelist_item_count',
                'taxes_id',
                'standard_price',
                'cost_method',
                'valuation',
                'company_id',
                'uom_categ_id',
                'uom_id',
                'category_id',
                'sh_is_secondary_unit',
                'sh_secondary_uom',
                'uom_po_id',
                'ephis_uom_id',
                'ephis_uom_moutp',
                'select_price_combine',
                'percent_combine',
                'combine_price',
                'currency_id',
                'cost_currency_id',
                'product_variant_id',
                'description',
                'invoice_policy',
                'service_type',
                'service_tracking',
                'project_id',
                'project_template_id',
                'visible_expense_policy',
                'expense_policy',
                'description_sale',
                'sale_line_warn',
                'sale_line_warn_msg',
                'seller_ids',
                'variant_seller_ids',
                'last_supplier_id',
                'last_purchase_date',
                'last_purchase_price',
                'service_to_purchase',
                'purchase_requisition',
                'supplier_taxes_id',
                'purchase_method',
                'description_purchase',
                'purchase_line_warn',
                'purchase_line_warn_msg',
                'has_available_route_ids',
                'route_ids',
                'route_from_categ_ids',
                'responsible_id',
                'weight',
                'weight_uom_name',
                'volume',
                'volume_uom_name',
                'produce_delay',
                'sale_delay',
                'tracking',
                'use_expiration_date',
                'property_stock_production',
                'property_stock_inventory',
                'expiration_time',
                'use_time',
                'removal_time',
                'alert_time',
                'packaging_ids',
                'description_pickingout',
                'description_pickingin',
                'description_picking',
                'property_account_income_id',
                'property_account_expense_id',
                'asset_category_id',
                'property_account_creditor_price_difference',
                'image_1',
                'image_2',
                'image_3',
                'image_4',
                'image_5',
                'image_6',
                'message_follower_ids',
                'activity_ids',
                'message_ids',
                'display_name',
              ],
            ],
            model: 'product.template',
            method: 'read',
            kwargs: {
              context: {
                bin_size: true,
                lang: 'th_TH',
                tz: 'Asia/Bangkok',
                uid: 10678,
                allowed_company_ids: [1],
                search_default_consumable: 1,
                default_type: 'product',
              },
            },
          },
        };
        const headers = {
          'Content-Type': 'application/json',
          Cookie: `session_id=${sessionId}`,
        };

        const response = await axios.post(odooURL, params, { headers });
        return response.data.result;
      }
      return null;
    } catch (error) {
      console.error('error getProductTemplate', error);
      throw new InternalServerErrorException(
        `Failed to fetch product template: ${error}`,
      );
    }
  }

  async searchProducts(searchText: string) {
    try {
      const { isValid, sessionId } = await authenticateOdoo();

      if (isValid && sessionId) {
        const odooURL = `${ODOO_BASE_URL}/web/dataset/search_read`;
        const params = {
          jsonrpc: '2.0',
          method: 'call',
          params: {
            model: 'product.template',
            domain: [
              '&',
              ['type', 'in', ['consu', 'product']],
              '|',
              '|',
              '|',
              ['product_variant_ids.default_code', 'ilike', searchText],
              ['name', 'ilike', searchText],
              ['barcode', 'ilike', searchText],
              ['sh_qr_code', 'ilike', searchText],
            ],
            fields: [
              'id',
              'product_variant_count',
              'currency_id',
              'activity_state',
              'name',
              'default_code',
              'lst_price',
              'immediately_usable_qty',
              'uom_id',
              'qty_available',
              'type',
            ],
            limit: 80,
            sort: '',
            context: {
              lang: 'th_TH',
              tz: 'Asia/Bangkok',
              uid: 10678,
              allowed_company_ids: [1],
              params: {
                action: 326,
                cids: 1,
                menu_id: 189,
                model: 'product.template',
                view_type: 'kanban',
              },
              search_default_consumable: 1,
              default_type: 'product',
              bin_size: true,
            },
          },
        };
        const headers = {
          'Content-Type': 'application/json',
          Cookie: `session_id=${sessionId}`,
        };

        const response = await axios.post(odooURL, params, { headers });
        return response.data.result;
      }
      return null;
    } catch (error) {
      console.error('error searchProducts', error);
      throw new InternalServerErrorException(
        `Failed to search products: ${error}`,
      );
    }
  }
}

const ODOO_BASE_URL = 'https://erpdev.vajira.ac.th';
const ODOO_DB = 'ERPDB1_230323';
const ODOO_USERNAME = 'ever';
const ODOO_PASSWORD = 'ever';

const authenticateOdoo = async () => {
  try {
    const response = await axios.post(
      ODOO_BASE_URL + '/web/session/authenticate',
      {
        jsonrpc: '2.0',
        params: {
          db: ODOO_DB,
          login: ODOO_USERNAME,
          password: ODOO_PASSWORD,
        },
      },
    );
    const data = response.data;
    const headers = response.headers['set-cookie'];

    if ('error' in data) {
      return { isValid: false, sessionId: null };
    }

    let sessionId: string | null = null;
    if (headers) {
      const cookieArray = headers[0].split('; ');
      for (const cookie of cookieArray) {
        const [cookieName, cookieValue] = cookie.split('=');
        if (cookieName === 'session_id') {
          sessionId = cookieValue;
          break;
        }
      }
    }

    return { isValid: true, sessionId };
  } catch (error) {
    console.error('error authenticate', error);
    throw error;
  }
};
