import * as v from 'valibot'

export type RegisterUserState = {
  fullName: string
  email: string
  password: string
  repeatPassword: string
}

export type RegisterUserResponse = {
  uuid: string
}

const isValidPassword = (password: string): boolean => {
  let containsUppercase = false
  let containsLowercase = false
  let containsNumber = false
  let containsSpecial = false

  const specialChars = ['!', '@', '#', '$', '%', '^', '&', '*']

  for (const char of password) {
    if (specialChars.includes(char)) {
      containsSpecial = true
    } else if (char >= '0' && char <= '9') {
      containsNumber = true
    } else if (char >= 'a' && char <= 'z') {
      containsLowercase = true
    } else if (char >= 'A' && char <= 'Z') {
      containsUppercase = true
    }

    if (containsLowercase && containsUppercase && containsNumber && containsSpecial) {
      return true
    }
  }

  return false
}

export const registerFormSchema = v.pipe(
  v.object({
    fullName: v.pipe(
      v.string(),
      v.minLength(3, 'Full name must be at least 3 characters'),
      v.maxLength(255, 'Full name cannot exceed 255 characters'),
    ),
    email: v.pipe(v.string(), v.email('Please enter a valid email address')),

    password: v.pipe(
      v.string(),
      v.check(
        isValidPassword,
        'Password should contain at least one lowercase and uppercase character, one number and one special character',
      ),
    ),
    repeatPassword: v.string(),
  }),

  v.forward(
    v.check((input) => input.password === input.repeatPassword, 'Passwords do not match'),
    ['repeatPassword'],
  ),
)

export type RegisterFormState = v.InferInput<typeof registerFormSchema>
