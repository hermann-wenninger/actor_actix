import axios, {AxiosResponse} from "axios";

async function handleRequest<T, X>( promise: Promise<AxiosResponse<X>>, expectedResponse: number) {
    let response: AxiosResponse<X>;
try {
    response = await promise;
} catch (error) {
    return {
        status: 500,
        error: 'Network or other error occurred'
    };
if (response.status === expectedResponse) {
    return {
        status: response.status,
        data: response.data as X
    };
} else {
    return {
        status: response.status,
        error: response.data as string
};
}
}



export async function postCall<T, X>(url: string, body: T,expectedResponse: number) {
    let response = axios.post<X | string>(
        url,
        body,
        {
            headers: {
                'Content-Type': 'application/json',
                'token': "jwt"
            },
            validateStatus: () => true
        });
    return handleRequest(response, expectedResponse);
}}