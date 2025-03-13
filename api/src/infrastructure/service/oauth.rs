use configurator::OAuthTrait;
use oauth2::{
    basic::{ BasicClient, BasicErrorResponseType, BasicTokenType },
    AuthUrl,
    ClientId,
    ClientSecret,
    EmptyExtraTokenFields,
    EndpointNotSet,
    EndpointSet,
    RedirectUrl,
    RevocationErrorResponseType,
    StandardErrorResponse,
    StandardRevocableToken,
    StandardTokenIntrospectionResponse,
    StandardTokenResponse,
    TokenUrl,
};

pub struct OAuthService {}

//  TE: ErrorResponse,
//     TR: TokenResponse,
//     TIR: TokenIntrospectionResponse,
//     RT: RevocableToken,
//     TRE: ErrorResponse,
//     HasAuthUrl: EndpointState,
//     HasDeviceAuthUrl: EndpointState,
//     HasIntrospectionUrl: EndpointState,
//     HasRevocationUrl: EndpointState,
//     HasTokenUrl: EndpointState,

impl OAuthService {
    pub fn client<T: OAuthTrait>(
        client_config: T
    ) -> oauth2::Client<
        StandardErrorResponse<BasicErrorResponseType>,
        StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
        StandardRevocableToken,
        StandardErrorResponse<RevocationErrorResponseType>,
        EndpointSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointSet
    > {
        let client_id = ClientId::new(client_config.client_id());
        let client_secret = ClientSecret::new(client_config.client_secret());
        let auth_url = AuthUrl::new(client_config.auth_url()).unwrap();
        let token_url = TokenUrl::new(client_config.token_url()).unwrap();

        BasicClient::new(client_id)
            .set_client_secret(client_secret)
            .set_auth_uri(auth_url)
            .set_token_uri(token_url)
            .set_redirect_uri(
                RedirectUrl::new("http://localhost:4001/oauth/authorize".to_string()).expect(
                    "Invalid redirect URL"
                )
            )
    }
}
