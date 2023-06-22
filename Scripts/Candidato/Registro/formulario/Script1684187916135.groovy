import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('Candidato/Registro/crear password'), [:], FailureHandling.STOP_ON_FAILURE)

fecha = ["2004-12-12","2003-12-12","2002-12-12","2001-12-12","2000-12-12","1999-12-12","1998-12-12","1997-12-12","1996-12-12","1995-12-12",
	"1994-12-12","1993-12-12","1992-12-12","1991-12-12","1990-12-12","1989-12-12","1988-12-12","1987-12-12","1986-12-12","1985-12-12","1984-12-12",
	"1983-12-12","1982-12-12",,"1981-12-12","1980-12-12","1979-12-12","1978-12-12","1977-12-12","1976-12-12","1975-12-12","1974-12-12",
	"1973-12-12","1972-12-12","1971-12-12","1970-12-12","1969-12-12","1968-12-12","1967-12-12","1966-12-12","1965-12-12","1964-12-12",
	"1963-12-12","1962-12-12","1961-12-12","1960-12-12"]

Nombres = ['Hugo', 'Paco', 'Luis', 'Carlos', 'Jorge', 'Alberto', 'Marcos', 'Victor', 'Raul', 'Juan']

ApellidoP = ['Álvarez', 'Castillo', 'De León', 'Díaz', 'Espinoza', 'Fernández', 'García', 'Salazar', 'Santana', 'Zambrano']

ApellidoM = ['Álvarez', 'Castillo', 'De León', 'Díaz', 'Espinoza', 'Fernández', 'García', 'Salazar', 'Santana', 'Zambrano']

Random rand = new Random()

int ranlist = rand.nextInt(fecha.size())

GlobalVariable.fecha = fecha.get(ranlist)

println(GlobalVariable.fecha)

int ranlist2 = rand.nextInt(Nombres.size())

GlobalVariable.Nombres = Nombres.get(ranlist2)

println(GlobalVariable.Nombres)



int ranlist3 = rand.nextInt(ApellidoP.size())

GlobalVariable.ApellidoP = ApellidoP.get(ranlist3)

println(GlobalVariable.ApellidoP)



int ranlist4 = rand.nextInt(ApellidoM.size())

GlobalVariable.ApellidoM = ApellidoM.get(ranlist4)

println(GlobalVariable.ApellidoM)


response = WS.sendRequest(findTestObject('candidato/registro/5.- formulario nombre'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

response = WS.sendRequest(findTestObject('candidato/registro/6.- formulario fecha'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

response = WS.sendRequest(findTestObject('candidato/registro/7.- genero'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

response = WS.sendRequest(findTestObject('candidato/registro/8.- Estado civil'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

response = WS.sendRequest(findTestObject('candidato/registro/9.- Nacionalidad'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

response = WS.sendRequest(findTestObject('candidato/registro/10.- primer Empleo'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

