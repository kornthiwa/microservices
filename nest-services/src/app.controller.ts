import { Controller, Get, Param } from '@nestjs/common';
import { AppService } from './app.service';

@Controller()
export class AppController {
  constructor(private readonly appService: AppService) {}

  @Get('product/:id')
  async getProduct(@Param('id') id: string): Promise<any> {
    console.log('id', id);
    const result = await this.appService.getProductTemplate(Number(id));
    console.log('result', result);
    return JSON.stringify(result);
  }

  @Get('product/search/:text')
  async searchProducts(@Param('text') text: string): Promise<any> {
    console.log('search text', text);
    const searchResult = await this.appService.searchProducts(text);
    console.log('search result', searchResult);

    if (searchResult?.records?.length > 0) {
      console.log('searchResult[0].id', searchResult.records[0].id);
      const details = await this.appService.getProductTemplate(
        searchResult.records[0].id,
      );
      return details?.[0] || searchResult.records[0];
    }
    return searchResult;
  }
}
