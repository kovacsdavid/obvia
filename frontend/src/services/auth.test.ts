/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2025 Kovács Dávid <kapcsolat@kovacsdavid.dev>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import {describe, expect, it, vi, beforeAll, afterEach, beforeEach} from "vitest";
import type {LoginRequest, RegisterRequest, LoginResponse, RegisterResponse} from "./auth";
import {isLoginResponse, isRegisterResponse, login, register} from "./auth";

describe("login", () => {
  const API_URL = import.meta.env.VITE_OBVIA_API_URL;

  beforeAll(() => {
    global.fetch = vi.fn();
  });

  beforeEach(() => {
    import.meta.env.VITE_OBVIA_API_URL = "http://localhost:3000"
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  it("should return valid response", async () => {
    const request: LoginRequest = {email: "user@example.com", password: "password123"};
    const mockResponse: LoginResponse = {
      success: true,
      data: {
        user: {id: "user123", email: "user@example.com"},
        token: "valid_token",
      },
    };
    vi.spyOn(global, "fetch").mockResolvedValueOnce({
      ok: true,
      json: async () => mockResponse,
    } as Response);

    const response = await login(request);

    expect(response).toEqual(mockResponse);
    expect(fetch).toHaveBeenCalledWith(`${API_URL}/login`, expect.objectContaining({
      method: "POST",
      headers: {"Content-Type": "application/json"},
      body: JSON.stringify(request),
    }));
  });

  it("should throw an error when API_URL is not set", async () => {
    delete import.meta.env.VITE_OBVIA_API_URL;

    const request: LoginRequest = {email: "user@example.com", password: "password123"};

    await expect(login(request)).rejects.toThrowError("API URL is not configured");
  });

  it("should throw error for invalid response", async () => {
    const request: LoginRequest = {email: "user@example.com", password: "password123"};
    const mockResponse  = {
      unknown_key: "unknown_value",
    };
    vi.spyOn(global, "fetch").mockResolvedValueOnce({
      ok: true,
      json: async () => mockResponse,
    } as Response);

    await expect(login(request)).rejects.toThrowError("Server responded with invalid data");
  });

  it("should throw an error for invalid JSON", async () => {
    const request: LoginRequest = {email: "user@example.com", password: "password123"};
    vi.spyOn(global, "fetch").mockResolvedValueOnce({
      ok: true,
      json: async () => {
        throw new Error("Unexpected token in JSON");
      },
    } as unknown as Response);

    await expect(login(request)).rejects.toThrowError("Server responded with invalid JSON format");
  });
});

describe("isLoginResponse", () => {
  it("should return true for a valid LoginResponse with success and data", () => {
    const validResponse: LoginResponse = {
      success: true,
      data: {
        user: {id: "user123", email: "user@example.com"},
        token: "valid_token",
      },
    };
    expect(isLoginResponse(validResponse)).toBe(true);
  });

  it("should return true for a valid LoginResponse with success and error", () => {
    const validResponseWithError: LoginResponse = {
      success: false,
      error: {
        reference: "ERR123",
        global: "Invalid credentials",
        fields: {email: "Invalid email"},
      },
    };
    expect(isLoginResponse(validResponseWithError)).toBe(true);
  });

  it("should return true for a valid LoginResponse with minimal fields", () => {
    const minimalResponse: LoginResponse = {
      success: true,
    };
    expect(isLoginResponse(minimalResponse)).toBe(true);
  });


  it("should return false for a response with missing success field", () => {
    const invalidResponse = {
      data: {
        user: {id: "user123", email: "user@example.com"},
        token: "valid_token",
      },
    };
    expect(isLoginResponse(invalidResponse)).toBe(false);
  });

  it("should return false for a response with invalid data structure", () => {
    const invalidDataResponse = {
      success: true,
      data: {
        user: {id: 123, email: "user@example.com"},
        token: "valid_token",
      },
    };
    expect(isLoginResponse(invalidDataResponse)).toBe(false);
  });

  it("should return false for a response with invalid error structure", () => {
    const invalidErrorResponse = {
      success: false,
      error: {
        global: 123,
        fields: {email: null},
      },
    };
    expect(isLoginResponse(invalidErrorResponse)).toBe(false);
  });

  it("should return false for completely invalid input", () => {
    const invalidInput = "This is not a valid response";
    expect(isLoginResponse(invalidInput)).toBe(false);
  });
});

describe("register", () => {
  const API_URL = import.meta.env.VITE_OBVIA_API_URL;

  beforeAll(() => {
    global.fetch = vi.fn();
  });

  beforeEach(() => {
    import.meta.env.VITE_OBVIA_API_URL = "http://localhost:3000"
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  it("should return valid response", async () => {
    const request: RegisterRequest = {
      firstName: "John",
      lastName: "Doe",
      email: "john@example.com",
      password: "password123",
      passwordConfirm: "password123"
    };
    const mockResponse: RegisterResponse = {
      success: true,
      data: {
        message: "Registration successful"
      }
    };
    vi.spyOn(global, "fetch").mockResolvedValueOnce({
      ok: true,
      json: async () => mockResponse,
    } as Response);

    const response = await register(request);

    expect(response).toEqual(mockResponse);
    expect(fetch).toHaveBeenCalledWith(`${API_URL}/register`, expect.objectContaining({
      method: "POST",
      headers: {"Content-Type": "application/json"},
      body: JSON.stringify({
        first_name: request.firstName,
        last_name: request.lastName,
        email: request.email,
        password: request.password,
        password_confirm: request.passwordConfirm
      }),
    }));
  });

  it("should throw an error when API_URL is not set", async () => {
    delete import.meta.env.VITE_OBVIA_API_URL;

    const request: RegisterRequest = {
      firstName: "John",
      lastName: "Doe",
      email: "john@example.com",
      password: "password123",
      passwordConfirm: "password123"
    };

    await expect(register(request)).rejects.toThrowError("API URL is not configured");
  });

  it("should throw error for invalid response", async () => {
    const request: RegisterRequest = {
      firstName: "John",
      lastName: "Doe",
      email: "invalid@example.com",
      password: "pass",
      passwordConfirm: "pass"
    };
    const mockResponse  = {
      unknown_key: "unknown_value",
    };
    vi.spyOn(global, "fetch").mockResolvedValueOnce({
      ok: true,
      json: async () => mockResponse,
    } as Response);

    await expect(register(request)).rejects.toThrowError("Server responded with invalid data");
  });

  it("should throw an error for invalid JSON", async () => {
    const request: RegisterRequest = {
      firstName: "John",
      lastName: "Doe",
      email: "john@example.com",
      password: "password123",
      passwordConfirm: "password123"
    };
    vi.spyOn(global, "fetch").mockResolvedValueOnce({
      ok: false,
      json: async () => {
        throw new Error("Unexpected token in JSON");
      },
    } as unknown as Response);

    await expect(register(request)).rejects.toThrowError("Server responded with invalid JSON format");
  });
});
describe("isRegisterResponse", () => {
  it("should return true for a valid RegisterResponse with success and message", () => {
    const validResponse: RegisterResponse = {
      success: true,
      data: {
        message: "Registration successful",
      },
    };
    expect(isRegisterResponse(validResponse)).toBe(true);
  });

  it("should return true for a valid RegisterResponse with success and no data or error", () => {
    const minimalValidResponse: RegisterResponse = {
      success: true,
    };
    expect(isRegisterResponse(minimalValidResponse)).toBe(true);
  });

  it("should return true for a valid RegisterResponse with an error object", () => {
    const errorResponse: RegisterResponse = {
      success: false,
      error: {
        reference: "ERR123",
        global: "Invalid input",
        fields: {
          email: "Email is invalid",
          username: null,
        },
      },
    };
    expect(isRegisterResponse(errorResponse)).toBe(true);
  });

  it("should return false for a response missing the success field", () => {
    const invalidResponse = {
      data: {
        message: "Registration successful",
      },
    };
    expect(isRegisterResponse(invalidResponse)).toBe(false);
  });

  it("should return false for a response with an invalid data structure", () => {
    const invalidDataResponse = {
      success: true,
      data: {
        message: 123,
      },
    };
    expect(isRegisterResponse(invalidDataResponse)).toBe(false);
  });

  it("should return false for a response with an invalid error structure", () => {
    const invalidErrorResponse = {
      success: false,
      error: {
        reference: 456,
        global: 789,
        fields: null,
      },
    };
    expect(isRegisterResponse(invalidErrorResponse)).toBe(false);
  });

  it("should return false for a completely invalid input", () => {
    const invalidInput = "Invalid response object";
    expect(isRegisterResponse(invalidInput)).toBe(false);
  });
});