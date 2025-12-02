# \ReferralApi

All URIs are relative to *https://api.ethereal.trade*

Method | HTTP request | Description
------------- | ------------- | -------------
[**referral_controller_activate**](ReferralApi.md#referral_controller_activate) | **POST** /v1/referral/activate | Activates the sender to acquire a referral code
[**referral_controller_claim_code**](ReferralApi.md#referral_controller_claim_code) | **POST** /v1/referral/claim | Claim a referral code
[**referral_controller_get_code_usage**](ReferralApi.md#referral_controller_get_code_usage) | **GET** /v1/referral/code/{code} | Returns referral code usage details
[**referral_controller_get_summary**](ReferralApi.md#referral_controller_get_summary) | **GET** /v1/referral/summary | Returns summary of your referral activity
[**referral_controller_list_referrals**](ReferralApi.md#referral_controller_list_referrals) | **GET** /v1/referral | Returns paginated list of referrals for the sender



## referral_controller_activate

> models::ReferralDto referral_controller_activate(activate_referral_dto)
Activates the sender to acquire a referral code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activate_referral_dto** | [**ActivateReferralDto**](ActivateReferralDto.md) |  | [required] |

### Return type

[**models::ReferralDto**](ReferralDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## referral_controller_claim_code

> models::ReferralDto referral_controller_claim_code(claim_referral_code_dto)
Claim a referral code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**claim_referral_code_dto** | [**ClaimReferralCodeDto**](ClaimReferralCodeDto.md) |  | [required] |

### Return type

[**models::ReferralDto**](ReferralDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## referral_controller_get_code_usage

> models::ReferralCodeUsageDto referral_controller_get_code_usage(code)
Returns referral code usage details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Referral code (3-12 alphanumeric uppercase characters) | [required] |

### Return type

[**models::ReferralCodeUsageDto**](ReferralCodeUsageDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## referral_controller_get_summary

> models::ReferralDto referral_controller_get_summary(x_ethereal_auth, x_ethereal_sender, x_ethereal_signature, x_ethereal_intent, x_ethereal_signed_at, subaccount)
Returns summary of your referral activity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_ethereal_auth** | **String** | Must be: EIP712Auth | [required] |
**x_ethereal_sender** | **String** | Address that signed this message (hex) | [required] |
**x_ethereal_signature** | **String** | The signature from signTypedData(...) signed by the sender | [required] |
**x_ethereal_intent** | **String** | Intent of the message (action to be taken) | [required] |
**x_ethereal_signed_at** | **String** | Message signedAt current timestamp (seconds since Unix Epoch) | [required] |
**subaccount** | Option<**String**> | Bytes32 encoded subaccount name (0x prefix, zero padded, set when using linked signer) |  |

### Return type

[**models::ReferralDto**](ReferralDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## referral_controller_list_referrals

> models::PageOfReferralDtos referral_controller_list_referrals(x_ethereal_auth, x_ethereal_sender, x_ethereal_signature, x_ethereal_intent, x_ethereal_signed_at, order, limit, cursor, subaccount, order_by)
Returns paginated list of referrals for the sender

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_ethereal_auth** | **String** | Must be: EIP712Auth | [required] |
**x_ethereal_sender** | **String** | Address that signed this message (hex) | [required] |
**x_ethereal_signature** | **String** | The signature from signTypedData(...) signed by the sender | [required] |
**x_ethereal_intent** | **String** | Intent of the message (action to be taken) | [required] |
**x_ethereal_signed_at** | **String** | Message signedAt current timestamp (seconds since Unix Epoch) | [required] |
**order** | Option<**String**> | Direction to paginate through objects |  |
**limit** | Option<**f64**> | Limit the number of objects to return |  |
**cursor** | Option<**String**> | Pointer to the current object in pagination dataset |  |
**subaccount** | Option<**String**> | Bytes32 encoded subaccount name (0x prefix, zero padded, set when using linked signer) |  |
**order_by** | Option<**String**> | Order by field |  |

### Return type

[**models::PageOfReferralDtos**](PageOfReferralDtos.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

