import { apiFetch } from './auth';
import { getApiConfig } from './config';

const API_BASE_URL = getApiConfig().baseUrl;

export const getUser = async (userId) => {
  const response = await apiFetch(`${API_BASE_URL}/users/${userId}`);
  return response;
};

export const updateUser = async (userId, userData) => {
  const response = await apiFetch(`${API_BASE_URL}/users/${userId}`, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(userData),
  });
  return response;
};

export const deleteUser = async (userId) => {
  const response = await apiFetch(`${API_BASE_URL}/users/${userId}`, {
    method: 'DELETE',
  });
  return response;
};
