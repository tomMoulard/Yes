import { jwtDecode } from "jwt-decode";

export const extractAndDecodeToken = () => {
  const token = localStorage.getItem('token');
  if (token) {
    return jwtDecode(token);
  }
  return null;
};

export const clearTokenAndRedirect = (history: any) => {
  localStorage.removeItem('token');
  history.push('/login');
};
