import { apiFetch } from './auth';
import { getApiConfig } from './config';

const API_BASE_URL = getApiConfig().baseUrl;

export const purchasePoints = async (points) => {
  const response = await apiFetch(`${API_BASE_URL}/purchase`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ points }),
  });
  return response;
};
