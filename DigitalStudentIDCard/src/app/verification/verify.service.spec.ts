import { TestBed } from '@angular/core/testing';
import {
  HttpClientTestingModule,
  HttpTestingController
} from '@angular/common/http/testing';
import { VerifyService } from './verify.service';
import { VerifyRes } from './verifyRes';
import { RouterModule } from '@angular/router';

describe('VerifyService', () => {
  beforeEach(() =>
    TestBed.configureTestingModule({
      imports: [HttpClientTestingModule,  RouterModule.forRoot([]),],
      providers: []
    })
  );

  it('should be created', () => {
    const service: VerifyService = TestBed.get(VerifyService);
    expect(service).toBeTruthy();
  });

  it('should perform a mocked http request', (done: DoneFn) => {
    const service: VerifyService = TestBed.get(VerifyService);
    const httpMock: HttpTestingController = TestBed.get(HttpTestingController);

    const mockResponse = {
        firstname: "Lars",
        lastname: "Born"
    };

    service.verify().subscribe((http: VerifyRes) => {
      expect(http).toBeTruthy();
      expect(http.firstname).toBe(mockResponse.firstname);
      expect(http.lastname).toBe(mockResponse.lastname);
      done();
    });

    const mockRequest = httpMock.expectOne(
      'http://10.231.16.32:3030/verify'
    );
    mockRequest.flush(mockResponse);
  });
});